pub mod tokenizer;
pub mod writer;

use tokenizer::Tokenizer;

use crate::source::Source;
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
            "Loc(line: {}, pos {}, bytes: {})",
            self.line, self.pos, self.bytes
        )
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Whilespace(Location),         // space
    Comment(String, Location),    // comment
    Keyword(Keyword, Location),   // keyword
    Symbol(String, Location),     // symbols
    Integer(u16, Location),       // integer constant
    Str(String, Location),        // string constant
    Identifier(String, Location), // identifier
}

#[derive(Debug, Clone, PartialEq, enum_utils::FromStr)]
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

fn to_lowercase_first_char(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    format!("{}{}", head.to_lowercase(), tail)
}

fn to_uppercase_first_char(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    format!("{}{}", head.to_uppercase(), tail)
}

fn keyword(s: &str) -> Option<Keyword> {
    let s = to_uppercase_first_char(s);
    s.parse::<Keyword>().ok()
}

pub fn tokenize(source: Rc<Source>) -> Result<Tokens> {
    println!("Start tokenize : {}", source.path.display());
    let tokenizer = Tokenizer::new(source.clone(), &source.content);

    let tokens: Result<Vec<Token>> = tokenizer.collect();
    let tokens = tokens?;

    println!("End tokenize : {}", source.path.display());
    Ok(Tokens {
        source: source.clone(),
        tokens: tokens,
    })
}

pub fn write_tokens(tokens: Tokens) -> Result<()> {
    writer::write_tokens(tokens)
}
