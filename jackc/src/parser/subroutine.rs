use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::{Keyword, Token};
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
            let modifier = modifier?;
            let typ = parse_return_type(stream)?;
            let name = stream.ensure_identifier()?;
            let parameters = parse_parameter_list(stream)?;
            let body = parse_subroutine_body(stream)?;

            let sub = SubroutineDec {
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

fn parse_subroutine_dec_modifier(stream: &mut Stream) -> Option<Result<SubroutineModifier>> {
    trace!(stream, "parse_subroutine_dec_modifier", {
        stream
            .consume_if_keywords(&[Keyword::Constructor, Keyword::Function, Keyword::Method])
            .map(|t| match t {
                Token::Keyword(Keyword::Constructor, _) => Ok(SubroutineModifier::Constructor),
                Token::Keyword(Keyword::Function, _) => Ok(SubroutineModifier::Function),
                Token::Keyword(Keyword::Method, _) => Ok(SubroutineModifier::Method),
                _ => stream.unexpected_token_result(
                    "expected keyword('constructor' or 'function' or 'method')",
                ),
            })
    });
}

fn parse_return_type(stream: &mut Stream) -> Result<ReturnType> {
    trace!(stream, "parse_return_type", {
        stream
            .consume_if_keyword(Keyword::Void)
            .map(|_t| Ok(ReturnType::Void))
            .or_else(|| Some(types::parse_type_or_die(stream).map(|ty| ReturnType::Type(ty))))
            .unwrap_or_else(|| stream.unexpected_token_result("expected type ('void' or type)"))
    });
}

fn parse_parameter_list(stream: &mut Stream) -> Result<Vec<ParameterDec>> {
    trace!(stream, "parse_parameter_list", {
        stream.ensure_symbol('(')?;
        let mut params = Vec::new();

        loop {
            match types::parse_type(stream) {
                Some(Ok(typ)) => {
                    let name = stream.ensure_identifier()?;
                    let param = ParameterDec { name, typ };
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
            let typ = types::parse_type_or_die(stream)?;
            let names = parse_var_dec_names(stream)?;

            stream.ensure_symbol(';')?;

            Ok(Some(VarDec { typ, names }))
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
