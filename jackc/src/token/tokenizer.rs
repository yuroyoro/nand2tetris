use crate::debug;
use crate::token::*;

use crate::source::Source;

use std::rc::Rc;

use anyhow::{anyhow, Result};

macro_rules! trace {
    ($self:ident, $msg: literal, $expression: expr) => {
        debug!("start : {} : {:?}", $msg, $self.iter.peek());

        let res = $expression;

        debug!("end : {} : {:?} -> {:?}", $msg, $self.iter.peek(), res);

        return res;
    };
}

pub struct Tokenizer<'a> {
    source: Rc<Source>, // reference to source file
    iter: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,  // current line number
    pos: usize,   // current byte position in line
    bytes: usize, // current byte postion in whole source file
}

impl Tokenizer<'_> {
    pub fn new<'a>(source: Rc<Source>, content: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            source: source.clone(),
            iter: content.char_indices().peekable(),
            line: 1,
            pos: 0,
            bytes: 0,
        }
    }

    fn next(&mut self) -> Option<(usize, char)> {
        if let Some((b, c)) = self.iter.next() {
            match c {
                '\n' => {
                    self.line += 1;
                    self.pos = 0;
                }
                _ => self.pos += 1,
            }
            self.bytes = b;
            Some((b, c))
        } else {
            None
        }
    }

    fn ensure<F>(&mut self, f: F)
    where
        F: FnOnce(char) -> bool,
    {
        self.consume_if(f).unwrap();
    }

    fn consume_if<F>(&mut self, f: F) -> Option<(char, Location)>
    where
        F: FnOnce(char) -> bool,
    {
        let current = self.iter.peek();
        if current.is_none() {
            return None;
        }

        let c = current.unwrap().1;
        if !f(c) {
            return None;
        }

        let loc = self.current_location();
        let c = self.next().unwrap().1;

        Some((c, loc))
    }

    fn consume_while<F>(&mut self, mut f: F) -> Option<(String, Location)>
    where
        F: FnMut(char, bool) -> bool,
    {
        let loc = self.current_location();
        let mut s = String::new();

        let mut escaped = false;
        while let Some((_pos, c)) = self.iter.peek() {
            if !f(*c, escaped) {
                break;
            }
            let c = self.next().unwrap().1;
            if c == '\\' {
                escaped = true;
                continue;
            }
            escaped = false;
            s.push(c);
        }

        if s.is_empty() {
            return None;
        }

        Some((s, loc))
    }

    fn consume_whitespaces(&mut self) -> TokenizeResult {
        trace!(
            self,
            "consume_whitespaces",
            self.consume_while(|c, _| c.is_ascii_whitespace())
                .map(|t| Ok(Token::Whilespace(t.1)))
        );
    }

    fn consume_comment(&mut self) -> TokenizeResult {
        trace!(
            self,
            "consume_comment",
            self.consume_if(|c| c == '/').and_then(|(_, loc)| {
                match self.iter.peek() {
                    // "//" comment, read until new line
                    Some((_, '/')) => self.consume_single_line_comment(loc),
                    // "/* .. */" comment, read until "*/"
                    Some((_, '*')) => self.consume_multi_line_comment(loc),
                    // signle "/" token
                    Some((_, _)) => Some(Ok(Token::Symbol('/', loc))),
                    None => None,
                }
            })
        );
    }

    fn consume_single_line_comment(&mut self, loc: Location) -> TokenizeResult {
        let mut comments = self
            .consume_while(|c, _| c != '\n')
            .map(|t| t.0)
            .unwrap_or("".to_string());
        comments.insert(0, '/');
        self.ensure(|c| c == '\n');
        comments.push('\n');

        Some(Ok(Token::Comment(comments, loc)))
    }

    fn consume_multi_line_comment(&mut self, loc: Location) -> TokenizeResult {
        // discard current char ('*'), or panic if it isn't.
        self.ensure(|c| c == '*');

        let mut comments = String::from("/");

        // read until "*/"
        loop {
            let s = self
                .consume_while(|c, _| c != '*')
                .map(|t| t.0)
                .unwrap_or("".to_string());

            comments.push_str(&s);

            if let Some(_) = self.consume_if(|c| c == '*') {
                comments.push('*');
            }

            match self.iter.peek() {
                Some((_, '/')) => break,
                Some((_, _)) => continue,
                None => break,
            }
        }

        // discard current char ('/'), or panic if it isn't.
        self.consume_if(|c| c == '/')
            .and_then(|_| {
                comments.push('/');
                Some(Ok(Token::Comment(comments, loc)))
            })
            .or(Some(Err(anyhow!("Tokenizer::string : unexpected EOF"))))
    }

    fn is_empty(&mut self) -> bool {
        self.iter.peek().is_none()
    }

    fn next_token(&mut self) -> TokenizeResult {
        // discard whiltespaces and comments
        loop {
            match self
                .consume_whitespaces()
                .or_else(|| self.consume_comment())
            {
                Some(Ok(Token::Whilespace(_))) => continue, // whilespace
                Some(Ok(Token::Comment(_, _))) => continue, // comment
                Some(res) => return Some(res),              // found token (maybe '/')
                None => break,
            }
        }

        // parse token
        self.symbol()
            .or_else(|| self.string())
            .or_else(|| self.keyword_or_identifier())
            .or_else(|| self.integer())
    }

    fn current_location(&self) -> Location {
        Location {
            source: self.source.clone(),
            line: self.line,
            pos: self.pos,
            bytes: self.bytes,
        }
    }

    fn symbol(&mut self) -> TokenizeResult {
        trace!(
            self,
            "symbol",
            self.consume_if(is_symbol_char)
                .map(|(c, loc)| Ok(Token::Symbol(c, loc)))
        );
    }

    fn string(&mut self) -> TokenizeResult {
        trace!(
            self,
            "string",
            self.consume_if(|c| c == '"').and_then(|(_, loc)| {
                let s = self
                    .consume_while(|c, escaped| escaped || (c != '"' && c != '\n'))
                    .map(|t| t.0)
                    .unwrap_or("".to_string());

                self.next()
                    .map(|(_, c)| match c {
                        '"' => Ok(Token::Str(s, loc)),
                        c => Err(anyhow!("Tokenizer::string : unexpected char '{}'", c)),
                    })
                    .or(Some(Err(anyhow!("Tokenizer::string : unexpected EOF"))))
            })
        );
    }

    fn keyword_or_identifier(&mut self) -> TokenizeResult {
        trace!(
            self,
            "keyword_or_identifier",
            self.consume_if(|c| c.is_ascii_alphabetic() || c == '_')
                .map(|(c, loc)| {
                    let mut s = self
                        .consume_while(|c, _| c.is_ascii_alphanumeric() || c == '_')
                        .map(|t| t.0)
                        .unwrap_or("".to_string());

                    s.insert(0, c);
                    let token = match keyword(&s) {
                        Some(kwd) => Token::Keyword(kwd, loc),
                        None => Token::Identifier(s, loc),
                    };
                    Ok(token)
                })
        );
    }

    fn integer(&mut self) -> TokenizeResult {
        trace!(
            self,
            "integer",
            self.consume_while(|c, _| c.is_ascii_alphanumeric() || c == '_')
                .map(|(s, loc)| {
                    s.parse::<u16>().map_err(|err| anyhow!(err)).and_then(|n| {
                        if n <= 32767 {
                            Ok(Token::Integer(n, loc))
                        } else {
                            Err(anyhow!(
                                "number must be grater equal than 0 and less equal than 32767 : {}",
                                n
                            ))
                        }
                    })
                })
        );
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Result<Token>;

    fn next(&mut self) -> TokenizeResult {
        if self.is_empty() {
            return None;
        }

        let res = self.next_token();
        debug!("  token -> {:?}", res);
        res
    }
}
