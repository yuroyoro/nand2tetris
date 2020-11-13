use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::{Keyword, Location, Token};
use anyhow::Result;

pub fn parse_type_or_die(stream: &mut Stream) -> Result<(Type, Location)> {
    parse_type(stream).unwrap_or_else(|| stream.unexpected_token_result("expected type"))
}

pub fn parse_type(stream: &mut Stream) -> Option<Result<(Type, Location)>> {
    stream
        .consume_if(|t| {
            t.is_keywords(&[Keyword::Int, Keyword::Char, Keyword::Boolean]) || t.is_identifier()
        })
        .map(|t| match t {
            Token::Keyword(Keyword::Int, loc) => Ok((Type::Int, loc)),
            Token::Keyword(Keyword::Char, loc) => Ok((Type::Char, loc)),
            Token::Keyword(Keyword::Boolean, loc) => Ok((Type::Boolean, loc)),
            Token::Identifier(ident, loc) => Ok((Type::Class(ident), loc)),
            _ => stream
                .unexpected_token_result("expected type ('int', 'char', 'boolean' or classname)"),
        })
}
