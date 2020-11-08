use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::Token;

use anyhow::Result;
use trace;

type RHS = (Op, Expr);

pub fn parse_op(stream: &mut Stream) -> Result<Option<RHS>> {
    trace!(stream, "parse_op", {
        parse_op_bool(stream)
            .or_else(|| parse_op_eq(stream))
            .or_else(|| parse_op_cmp(stream))
            .or_else(|| parse_op_add(stream))
            .or_else(|| parse_op_mul(stream))
            .transpose()
    });
}

fn extract_op(token: Token, stream: &mut Stream, expected_ops: &[Op], msg: &str) -> Result<Op> {
    token
        .symbol()
        .map(|sym| {
            Op::parse(sym)
                .filter(|op| expected_ops.iter().any(|eop| op == eop))
                .ok_or_else(|| stream.unexpected_token_err(msg))
        })
        .unwrap_or_else(|| stream.unexpected_token_result(msg))
}

fn parse_op_bool(stream: &mut Stream) -> Option<Result<RHS>> {
    trace!(stream, "parse_op_bool", {
        stream.consume_if_symbols(&['|', '&']).map(|t| {
            extract_op(
                t,
                stream,
                &[Op::Or, Op::And],
                "expected boolean operator('|', '&')",
            )
            .and_then(|op| expr::parse_expr_or_die(stream).map(|rhs| (op, rhs)))
        })
    });
}

fn parse_op_eq(stream: &mut Stream) -> Option<Result<RHS>> {
    trace!(stream, "parse_op_eq", {
        stream.consume_if_symbol('=').map(|t| {
            extract_op(t, stream, &[Op::Eq], "expected eq operator('=')")
                .and_then(|op| expr::parse_expr_or_die(stream).map(|rhs| (op, rhs)))
        })
    });
}

fn parse_op_cmp(stream: &mut Stream) -> Option<Result<RHS>> {
    trace!(stream, "parse_op_cmp", {
        stream.consume_if_symbols(&['<', '>']).map(|t| {
            extract_op(
                t,
                stream,
                &[Op::Lt, Op::Gt],
                "expected cmp operator('<', '>')",
            )
            .and_then(|op| expr::parse_expr_or_die(stream).map(|rhs| (op, rhs)))
        })
    });
}

fn parse_op_add(stream: &mut Stream) -> Option<Result<RHS>> {
    trace!(stream, "parse_op_add", {
        stream.consume_if_symbols(&['+', '-']).map(|t| {
            extract_op(
                t,
                stream,
                &[Op::Add, Op::Sub],
                "expected add or sub operator('+', '-')",
            )
            .and_then(|op| expr::parse_expr_or_die(stream).map(|rhs| (op, rhs)))
        })
    });
}

fn parse_op_mul(stream: &mut Stream) -> Option<Result<RHS>> {
    trace!(stream, "parse_op_mul", {
        stream.consume_if_symbols(&['*', '/']).map(|t| {
            extract_op(
                t,
                stream,
                &[Op::Mul, Op::Div],
                "expected mul or div operator('*', '/')",
            )
            .and_then(|op| expr::parse_expr_or_die(stream).map(|rhs| (op, rhs)))
        })
    });
}
