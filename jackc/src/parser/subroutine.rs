use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::{Keyword, Location, Token};
use anyhow::Result;
use trace;

pub fn parse_subroutine_decs(stream: &mut Stream) -> Result<Vec<SubroutineDec>> {
    trace!(stream, "parse_subroutine_decs", {
        let mut subs = Vec::new();

        loop {
            if let Some(sub) = parse_subroutine_dec(stream)? {
                subs.push(sub);
            } else {
                break;
            }
        }

        Ok(subs)
    });
}

fn parse_subroutine_dec(stream: &mut Stream) -> Result<Option<SubroutineDec>> {
    trace!(stream, "parse_subroutine_dec", {
        if let Some(modifier) = parse_subroutine_dec_modifier(stream) {
            let (modifier, loc) = modifier?;
            let (typ, _loc) = parse_return_type(stream)?;
            let name = stream.ensure_identifier()?;
            let parameters = parse_parameter_list(stream)?;
            let body = parse_subroutine_body(stream)?;

            let sub = SubroutineDec {
                loc,
                modifier,
                typ,
                name,
                parameters,
                body,
            };

            Ok(Some(sub))
        } else {
            Ok(None)
        }
    });
}

fn parse_subroutine_dec_modifier(
    stream: &mut Stream,
) -> Option<Result<(SubroutineModifier, Location)>> {
    trace!(stream, "parse_subroutine_dec_modifier", {
        stream
            .consume_if_keywords(&[Keyword::Constructor, Keyword::Function, Keyword::Method])
            .map(|t| match t {
                Token::Keyword(Keyword::Constructor, loc) => {
                    Ok((SubroutineModifier::Constructor, loc))
                }
                Token::Keyword(Keyword::Function, loc) => Ok((SubroutineModifier::Function, loc)),
                Token::Keyword(Keyword::Method, loc) => Ok((SubroutineModifier::Method, loc)),
                _ => stream.unexpected_token_result(
                    "expected keyword('constructor' or 'function' or 'method')",
                ),
            })
    });
}

fn parse_return_type(stream: &mut Stream) -> Result<(ReturnType, Location)> {
    trace!(stream, "parse_return_type", {
        stream
            .consume_if_keyword(Keyword::Void)
            .map(|t| Ok((ReturnType::Void, t.location())))
            .or_else(|| {
                Some(types::parse_type_or_die(stream).map(|(ty, loc)| (ReturnType::Type(ty), loc)))
            })
            .unwrap_or_else(|| stream.unexpected_token_result("expected type ('void' or type)"))
    });
}

fn parse_parameter_list(stream: &mut Stream) -> Result<Vec<ParameterDec>> {
    trace!(stream, "parse_parameter_list", {
        stream.ensure_symbol('(')?;
        let mut params = Vec::new();

        loop {
            match types::parse_type(stream) {
                Some(Ok((typ, loc))) => {
                    let name = stream.ensure_identifier()?;
                    let param = ParameterDec { loc, name, typ };
                    params.push(param);
                }
                Some(Err(err)) => return Err(err),
                None => break,
            }

            if stream.consume_if_symbol(',').is_none() {
                break; // end of parameter list
            }
        }

        stream.ensure_symbol(')')?;

        Ok(params)
    });
}

fn parse_subroutine_body(stream: &mut Stream) -> Result<SubroutineBody> {
    trace!(stream, "parse_subroutine_body", {
        stream.ensure_symbol('{')?;

        let vars = parse_vars(stream)?;
        let statements = statement::parse_statements(stream)?;

        stream.ensure_symbol('}')?;

        let body = SubroutineBody { vars, statements };
        Ok(body)
    });
}

fn parse_vars(stream: &mut Stream) -> Result<Vec<VarDec>> {
    trace!(stream, "parse_var_decs", {
        let mut vars = Vec::new();

        loop {
            if let Some(vardec) = parse_var_dec(stream)? {
                vars.push(vardec);
            } else {
                break;
            }
        }

        Ok(vars)
    });
}

fn parse_var_dec(stream: &mut Stream) -> Result<Option<VarDec>> {
    trace!(stream, "parse_var_dec", {
        if let Some(_) = stream.consume_if_keyword(Keyword::Var) {
            let (typ, loc) = types::parse_type_or_die(stream)?;
            let names = parse_var_dec_names(stream)?;

            stream.ensure_symbol(';')?;

            Ok(Some(VarDec { loc, typ, names }))
        } else {
            Ok(None)
        }
    });
}

fn parse_var_dec_names(stream: &mut Stream) -> Result<Vec<String>> {
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
