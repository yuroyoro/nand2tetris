#[macro_use]
pub mod macros;

pub mod ast;
pub mod class;
pub mod expr;
pub mod op;
pub mod statement;
pub mod stream;
pub mod subroutine;
pub mod term;
pub mod types;
pub mod writer;

use crate::token::Tokens;

use ast::*;
use stream::Stream;

use anyhow::Result;

pub fn parse(tokens: Tokens) -> Result<ASTs> {
    let source = tokens.source.clone();
    debug!("==== Start : parse : {}", source.path.display());
    let mut stream = Stream::new(tokens);

    let class = class::parse_class(&mut stream)?;

    debug!("==== End : parse : {}\n", source.path.display());

    Ok(ASTs { source, class })
}

pub fn write_asts(asts: ASTs) -> Result<()> {
    writer::write_asts(asts)
}
