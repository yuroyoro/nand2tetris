use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::{Keyword, Location, Token};
use trace;

use anyhow::Result;

// ’class’ className ’{’ classVarDec* subroutineDec* ’}’
pub fn parse_class(stream: &mut Stream) -> Result<Class> {
    trace!(stream, "parse_class", {
        let loc = stream.location()?;
        stream.ensure_keyword(Keyword::Class)?;

        let name = stream.ensure_identifier()?;

        stream.ensure_symbol('{')?;

        let vars = parse_class_var_decs(stream)?;
        let subroutines = subroutine::parse_subroutine_decs(stream)?;

        Ok(Class {
            loc,
            name,
            vars,
            subroutines,
        })
    });
}

fn parse_class_var_decs(stream: &mut Stream) -> Result<Vec<ClassVarDec>> {
    trace!(stream, "parse_class_var_decs", {
        let mut vars = Vec::new();

        loop {
            if let Some(vardec) = parse_class_var_dec(stream)? {
                vars.push(vardec);
            } else {
                break;
            }
        }

        Ok(vars)
    });
}

fn parse_class_var_dec(stream: &mut Stream) -> Result<Option<ClassVarDec>> {
    trace!(stream, "parse_class_var_dec", {
        if let Some(modifier) = parse_class_var_dec_modifier(stream) {
            let (modifier, loc) = modifier?;
            let (typ, _loc) = types::parse_type_or_die(stream)?;
            let names = parse_class_var_dec_names(stream)?;

            stream.ensure_symbol(';')?;
            let cls = ClassVarDec {
                loc,
                modifier,
                typ,
                names,
            };
            Ok(Some(cls))
        } else {
            Ok(None)
        }
    });
}

fn parse_class_var_dec_modifier(
    stream: &mut Stream,
) -> Option<Result<(ClassVarModifier, Location)>> {
    stream
        .consume_if_keywords(&[Keyword::Static, Keyword::Field])
        .map(|t| match t {
            Token::Keyword(Keyword::Static, loc) => Ok((ClassVarModifier::Static, loc)),
            Token::Keyword(Keyword::Field, loc) => Ok((ClassVarModifier::Field, loc)),
            _ => stream.unexpected_token_result("expected keyword('static' or 'field')"),
        })
}

fn parse_class_var_dec_names(stream: &mut Stream) -> Result<Vec<String>> {
    let mut names = vec![stream.ensure_identifier()?];

    loop {
        if !stream.is_symbol(',') {
            break;
        }
        let name = stream.ensure_symbol(',').and(stream.ensure_identifier())?;
        names.push(name);
    }

    Ok(names)
}
