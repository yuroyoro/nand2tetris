use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::{Keyword, Token};

use anyhow::Result;
use trace;

pub fn parse_term_or_die(stream: &mut Stream) -> Result<Term> {
    parse_term(stream).unwrap_or_else(|| stream.unexpected_token_result("expected term"))
}

pub fn parse_term(stream: &mut Stream) -> Option<Result<Term>> {
    trace!(stream, "parse_term", {
        parse_term_integer_const(stream)
            .or_else(|| parse_term_string_const(stream))
            .or_else(|| parse_term_keyword_const(stream))
            .or_else(|| parse_term_var(stream))
            .or_else(|| parse_term_expr(stream))
            .or_else(|| parse_term_unary(stream))
    });
}

fn parse_term_integer_const(stream: &mut Stream) -> Option<Result<Term>> {
    trace!(stream, "parse_term_integer_const", {
        stream.consume_if_integer().map(|t| {
            t.integer()
                .map(|n| Term::Integer(n))
                .ok_or_else(|| stream.unexpected_token_err("expected integer"))
        })
    });
}

fn parse_term_string_const(stream: &mut Stream) -> Option<Result<Term>> {
    trace!(stream, "parse_term_string_const", {
        stream.consume_if_str().map(|t| {
            t.str()
                .map(|n| Term::Str(n))
                .ok_or_else(|| stream.unexpected_token_err("expected string"))
        })
    });
}

fn parse_term_keyword_const(stream: &mut Stream) -> Option<Result<Term>> {
    trace!(stream, "parse_term_keyword_const", {
        stream
            .consume_if_keywords(&[Keyword::True, Keyword::False, Keyword::Null, Keyword::This])
            .map(|t| match t.keyword() {
                Some(Keyword::True) => Ok(Term::Keyword(KeywordConst::True)),
                Some(Keyword::False) => Ok(Term::Keyword(KeywordConst::False)),
                Some(Keyword::Null) => Ok(Term::Keyword(KeywordConst::Null)),
                Some(Keyword::This) => Ok(Term::Keyword(KeywordConst::This)),
                _ => stream
                    .unexpected_token_result("expected keyword('true', 'false', 'null', 'this')"),
            })
    });
}

fn parse_term_var(stream: &mut Stream) -> Option<Result<Term>> {
    trace!(stream, "parse_term_var_or_call", {
        stream.consume_if_identifier().map(|t| {
            t.identifier()
                .map(|ident| {
                    match stream.current() {
                        // index access
                        Some(Token::Symbol('[', _)) => parse_term_index_access(ident, stream),
                        Some(Token::Symbol('(', _)) => parse_term_call(ident, stream),
                        Some(Token::Symbol('.', _)) => parse_term_call(ident, stream),
                        _ => Ok(Term::Var(ident)),
                    }
                })
                .unwrap_or_else(|| stream.unexpected_token_result("expected identifier"))
        })
    });
}

fn parse_term_index_access(ident: String, stream: &mut Stream) -> Result<Term> {
    stream.ensure_symbol('[')?;
    let expr = expr::parse_expr_or_die(stream)?;

    stream.ensure_symbol(']')?;

    Ok(Term::IndexAccess(ident, expr))
}

fn parse_term_call(ident: String, stream: &mut Stream) -> Result<Term> {
    trace!(stream, "parse_term_call", {
        let call = expr::parse_subroutine_call_with_ident(ident, stream)?;
        Ok(Term::Call(call))
    });
}

fn parse_term_expr(stream: &mut Stream) -> Option<Result<Term>> {
    trace!(stream, "parse_term_expr", {
        stream.consume_if_symbol('(').map(|_t| {
            expr::parse_expr_or_die(stream)
                .and_then(|expr| stream.ensure_symbol(')').and(Ok(Term::Expr(expr))))
        })
    });
}

fn parse_term_unary(stream: &mut Stream) -> Option<Result<Term>> {
    trace!(stream, "parse_term_unary", {
        stream.consume_if_symbols(&['-', '~']).map(|t| {
            t.symbol()
                .map(|sym| {
                    let unary = match sym {
                        '-' => Ok(UnaryOp::Minus),
                        '~' => Ok(UnaryOp::Not),
                        _ => stream.unexpected_token_result("expected unary ('-', '~')"),
                    }?;

                    let term = parse_term_or_die(stream)?;
                    Ok(Term::Unary(unary, Box::new(term)))
                })
                .unwrap_or_else(|| stream.unexpected_token_result("expected unary ('-', '~')"))
        })
    });
}
