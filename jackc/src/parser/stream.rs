use crate::token::{Keyword, Location, Token, Tokens};

use anyhow::{anyhow, Result};

pub struct Stream {
    iter: std::iter::Peekable<std::vec::IntoIter<Token>>,
}

// Token Stream
impl Stream {
    pub fn new(tokens: Tokens) -> Stream {
        let iter = tokens.tokens.into_iter().peekable();
        Stream { iter }
    }

    pub fn current(&mut self) -> Option<&Token> {
        self.iter.peek()
    }

    pub fn location(&mut self) -> Result<Location> {
        self.iter
            .peek()
            .map(|t| t.location())
            .ok_or_else(|| anyhow!("unexpected eof",))
    }

    pub fn ensure<F>(&mut self, msg: &str, f: F) -> Result<Token>
    where
        F: FnOnce(&Token) -> bool,
    {
        self.consume_if(f)
            .ok_or_else(|| self.unexpected_token_err(msg))
    }

    pub fn consume_if<F>(&mut self, f: F) -> Option<Token>
    where
        F: FnOnce(&Token) -> bool,
    {
        if self.current().is_none() {
            return None;
        }

        let token = self.current().unwrap();
        if !f(token) {
            return None;
        }

        self.next()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.iter.next()
    }

    // symbol
    pub fn is_symbol(&mut self, sym: char) -> bool {
        self.is_symbols(&[sym])
    }

    pub fn is_symbols(&mut self, syms: &[char]) -> bool {
        self.current().map(|t| t.is_symbols(syms)).unwrap_or(false)
    }

    pub fn ensure_symbol(&mut self, sym: char) -> Result<Token> {
        self.ensure_symbols(&[sym])
    }

    pub fn ensure_symbols(&mut self, syms: &[char]) -> Result<Token> {
        self.ensure(format!("expected symbol({:?})", syms).as_str(), |t| {
            t.is_symbols(syms)
        })
    }

    pub fn consume_if_symbol(&mut self, sym: char) -> Option<Token> {
        self.consume_if_symbols(&[sym])
    }

    pub fn consume_if_symbols(&mut self, syms: &[char]) -> Option<Token> {
        self.consume_if(|t| t.is_symbols(syms))
    }

    // keyword
    pub fn is_keyword(&mut self, kwd: Keyword) -> bool {
        self.is_keywords(&[kwd])
    }

    pub fn is_keywords(&mut self, kwds: &[Keyword]) -> bool {
        self.current().map(|t| t.is_keywords(kwds)).unwrap_or(false)
    }

    pub fn ensure_keyword(&mut self, kwd: Keyword) -> Result<Token> {
        self.ensure_keywords(&[kwd])
    }

    pub fn ensure_keywords(&mut self, kwds: &[Keyword]) -> Result<Token> {
        self.ensure(format!("expected keyword({:?})", kwds).as_str(), |t| {
            t.is_keywords(kwds)
        })
    }

    pub fn consume_if_keyword(&mut self, kwd: Keyword) -> Option<Token> {
        self.consume_if_keywords(&[kwd])
    }

    pub fn consume_if_keywords(&mut self, kwds: &[Keyword]) -> Option<Token> {
        self.consume_if(|t| t.is_keywords(kwds))
    }

    // integer
    pub fn is_integer(&mut self) -> bool {
        self.current().map(|t| t.is_integer()).unwrap_or(false)
    }

    pub fn ensure_integer(&mut self) -> Result<u16> {
        let n = self.ensure("expected integer", |t| t.is_integer())?;

        n.integer()
            .ok_or_else(|| self.unexpected_token_err("expected integer"))
    }

    pub fn consume_if_integer(&mut self) -> Option<Token> {
        self.consume_if(|t| t.is_integer())
    }

    // str
    pub fn is_str(&mut self) -> bool {
        self.current().map(|t| t.is_str()).unwrap_or(false)
    }

    pub fn ensure_str(&mut self) -> Result<String> {
        let n = self.ensure("expected str", |t| t.is_str())?;

        n.str()
            .ok_or_else(|| self.unexpected_token_err("expected str"))
    }

    pub fn consume_if_str(&mut self) -> Option<Token> {
        self.consume_if(|t| t.is_str())
    }

    // identifier
    pub fn is_identifier(&mut self) -> bool {
        self.current().map(|t| t.is_identifier()).unwrap_or(false)
    }

    pub fn ensure_identifier(&mut self) -> Result<String> {
        let ident = self.ensure("expected identifier", |t| t.is_identifier())?;

        ident
            .identifier()
            .ok_or_else(|| self.unexpected_token_err("expected identifier"))
    }

    pub fn consume_if_identifier(&mut self) -> Option<Token> {
        self.consume_if(|t| t.is_identifier())
    }

    // errors
    pub fn unexpected_token_result<T>(&mut self, msg: &str) -> Result<T> {
        Err(self.unexpected_token_err(msg))
    }

    pub fn unexpected_token_err(&mut self, msg: &str) -> anyhow::Error {
        anyhow!(
            "unexpected token : current {} : {}",
            self.current()
                .map(|t| format!("{:?}", t))
                .unwrap_or_else(|| "EOF".to_string()),
            msg
        )
    }
}
