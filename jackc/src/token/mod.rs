pub mod tokenizer;
pub mod writer;

use tokenizer::Tokenizer;

use crate::source::Source;
use crate::{to_lowercase_first_char, to_uppercase_first_char};
use std::fmt;
use std::rc::Rc;

use anyhow::Result;

#[derive(Debug)]
pub struct Tokens {
    pub source: Rc<Source>,
    pub tokens: Vec<Token>,
}

#[derive(Clone)]
pub struct Location {
    pub source: Rc<Source>, // reference to source file
    pub line: usize,        // line number
    pub pos: usize,         // byte position in line
    pub bytes: usize,       // byte postion in whole source file
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.source.path.display(),
            self.line,
            self.pos,
        )
    }
}

#[derive(Clone)]
pub enum Token {
    Whilespace(Location),         // space
    Comment(String, Location),    // comment
    Keyword(Keyword, Location),   // keyword
    Symbol(char, Location),       // symbols
    Integer(u16, Location),       // integer constant
    Str(String, Location),        // string constant
    Identifier(String, Location), // identifier
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Whilespace(loc) => write!(f, "Whilespace : {:?}", loc),
            Token::Comment(cmt, loc) => write!(f, "Comment({}) : {:?}", cmt, loc),
            Token::Keyword(kwd, loc) => write!(f, "Keyword({:?}) : {:?}", kwd, loc),
            Token::Symbol(sym, loc) => write!(f, "Symbol({}) : {:?}", sym, loc),
            Token::Integer(n, loc) => write!(f, "Identifier({}) : {:?}", n, loc),
            Token::Str(s, loc) => write!(f, "Str({}) : {:?}", s, loc),
            Token::Identifier(ident, loc) => write!(f, "Identifier({}) : {:?}", ident, loc),
        }
    }
}

impl Token {
    pub fn location(&self) -> Location {
        match self {
            Token::Whilespace(loc) => loc.clone(),
            Token::Comment(_, loc) => loc.clone(),
            Token::Keyword(_, loc) => loc.clone(),
            Token::Symbol(_, loc) => loc.clone(),
            Token::Integer(_, loc) => loc.clone(),
            Token::Str(_, loc) => loc.clone(),
            Token::Identifier(_, loc) => loc.clone(),
        }
    }

    pub fn is_any_keyword(&self) -> bool {
        match self {
            Token::Keyword(_, _) => true,
            _ => false,
        }
    }

    pub fn is_keyword(&self, kwd: Keyword) -> bool {
        match self {
            Token::Keyword(k, _) => *k == kwd,
            _ => false,
        }
    }

    pub fn is_keywords(&self, kwds: &[Keyword]) -> bool {
        kwds.iter().any(|kwd| match self {
            Token::Keyword(k, _) => *k == *kwd,
            _ => false,
        })
    }

    pub fn keyword(&self) -> Option<Keyword> {
        match self {
            Token::Keyword(k, _) => Some(*k),
            _ => None,
        }
    }

    pub fn is_any_symbol(&self) -> bool {
        match self {
            Token::Symbol(_, _) => true,
            _ => false,
        }
    }

    pub fn is_symbol(&self, c: char) -> bool {
        match self {
            Token::Symbol(sym, _) => *sym == c,
            _ => false,
        }
    }

    pub fn is_symbols(&self, syms: &[char]) -> bool {
        syms.iter().any(|sym| self.is_symbol(*sym))
    }

    pub fn symbol(&self) -> Option<char> {
        match self {
            Token::Symbol(sym, _) => Some(*sym),
            _ => None,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Token::Integer(_, _) => true,
            _ => false,
        }
    }

    pub fn integer(&self) -> Option<u16> {
        match self {
            Token::Integer(i, _) => Some(*i),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Token::Str(_, _) => true,
            _ => false,
        }
    }

    pub fn str(&self) -> Option<String> {
        match self {
            Token::Str(s, _) => Some(s.to_string()),
            _ => None,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier(_, _) => true,
            _ => false,
        }
    }

    pub fn identifier(&self) -> Option<String> {
        match self {
            Token::Identifier(ident, _) => Some(ident.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, enum_utils::FromStr)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

type TokenizeResult = Option<Result<Token>>;

fn is_symbol_char(c: char) -> bool {
    match c {
        '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | ';' | '+' | '-' | '*' | '/' | '&' | '|'
        | '<' | '>' | '=' | '~' => true,
        _ => false,
    }
}

fn keyword(s: &str) -> Option<Keyword> {
    let s = to_uppercase_first_char(s);
    s.parse::<Keyword>().ok()
}

pub fn tokenize(source: Rc<Source>) -> Result<Tokens> {
    debug!("Start tokenize : {}", source.path.display());
    let tokenizer = Tokenizer::new(source.clone(), &source.content);

    let tokens: Result<Vec<Token>> = tokenizer.collect();
    let tokens = tokens?;

    debug!("End tokenize : {}\n", source.path.display());
    Ok(Tokens {
        source: source.clone(),
        tokens: tokens,
    })
}

pub fn write_tokens(tokens: Tokens) -> Result<()> {
    writer::write_tokens(tokens)
}
