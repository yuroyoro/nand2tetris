use crate::parser::stream::Stream;
use crate::parser::*;

use anyhow::Result;
use trace;

pub fn parse_expr_or_die(stream: &mut Stream) -> Result<Expr> {
    parse_expr(stream).unwrap_or_else(|| stream.unexpected_token_result("expected expression"))
}

pub fn parse_expr(stream: &mut Stream) -> Option<Result<Expr>> {
    trace!(stream, "parse_expr", {
        term::parse_term(stream).map(|lhs| {
            let (lhs, loc) = lhs?;
            let rhs = op::parse_op(stream)?;
            let expr = Expr {
                loc: loc,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
            Ok(expr)
        })
    });
}

pub fn parse_expr_list(stream: &mut Stream) -> Result<Vec<Expr>> {
    let mut exprs = Vec::new();

    while let Some(expr) = parse_expr(stream) {
        exprs.push(expr?);
        if stream.consume_if_symbol(',').is_none() {
            break;
        }
    }

    Ok(exprs)
}

pub fn parse_subroutine_call(stream: &mut Stream) -> Result<SubroutineCall> {
    trace!(stream, "parse_subroutine_call", {
        let name = stream.ensure_identifier()?;
        parse_subroutine_call_with_ident(name, stream)
    });
}

pub fn parse_subroutine_call_with_ident(
    name: String,
    stream: &mut Stream,
) -> Result<SubroutineCall> {
    trace!(stream, "parse_subroutine_call_with_ident", {
        let loc = stream.location()?;
        let (reciever, name): (Option<String>, String) = match stream.consume_if_symbol('.') {
            Some(_) => stream
                .ensure_identifier()
                .map(|target| (Some(name), target)),
            None => Ok((None, name)),
        }?;

        stream.ensure_symbol('(')?;

        let exprs = parse_expr_list(stream)?;

        stream.ensure_symbol(')')?;

        Ok(SubroutineCall {
            loc,
            reciever,
            name,
            exprs,
        })
    });
}
