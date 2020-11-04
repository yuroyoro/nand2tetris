use crate::source::Source;
use crate::tokenizer::Tokens;

use std::rc::Rc;

use anyhow::Result;

#[derive(Debug)]
pub struct ASTs {
    pub source: Rc<Source>,
    pub asts: Vec<AST>,
}

#[derive(Debug, Clone)]
pub enum AST {}

pub fn parse(tokens: Tokens) -> Result<ASTs> {
    let asts = Vec::new();

    Ok(ASTs {
        source: tokens.source,
        asts: asts,
    })
}

pub fn write_asts(asts: ASTs) -> () {
    asts.asts.iter().for_each(|ast| println!("{:?}", ast));
}
