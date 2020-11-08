use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::{Keyword, Token};
use anyhow::Result;

pub fn parse_type_or_die(stream: &mut Stream) -> Result<Type> {
    parse_type(stream).unwrap_or_else(|| stream.unexpected_token_result("expected type"))
}

pub fn parse_type(stream: &mut Stream) -> Option<Result<Type>> {
    stream
        .consume_if(|t| {
            t.is_keywords(&[Keyword::Int, Keyword::Char, Keyword::Boolean]) || t.is_identifier()
        })
        .map(|t| match t {
            Token::Keyword(Keyword::Int, _) => Ok(Type::Int),
            Token::Keyword(Keyword::Char, _) => Ok(Type::Char),
            Token::Keyword(Keyword::Boolean, _) => Ok(Type::Boolean),
            Token::Identifier(ident, _) => Ok(Type::Class(ident)),
            _ => stream
                .unexpected_token_result("expected type ('int', 'char', 'boolean' or classname)"),
        })
}
