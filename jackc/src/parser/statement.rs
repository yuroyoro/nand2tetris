use crate::parser::stream::Stream;
use crate::parser::*;
use crate::token::Keyword;

use anyhow::Result;
use trace;

pub fn parse_statements(stream: &mut Stream) -> Result<Statements> {
    trace!(stream, "parse_statements", {
        let mut statements = Vec::new();

        loop {
            if let Some(vardec) = parse_statement(stream)? {
                statements.push(vardec);
            } else {
                break;
            }
        }

        Ok(Statements { statements })
    });
}

fn parse_statement(stream: &mut Stream) -> Result<Option<Statement>> {
    trace!(stream, "parse_statements", {
        parse_let(stream)
            .or_else(|| parse_if(stream))
            .or_else(|| parse_while(stream))
            .or_else(|| parse_do(stream))
            .or_else(|| parse_return(stream))
            .transpose()
    });
}

fn parse_let(stream: &mut Stream) -> Option<Result<Statement>> {
    trace!(stream, "parse_let", {
        stream.consume_if_keyword(Keyword::Let).map(|_t| {
            let name = stream.ensure_identifier()?;
            let accessor = parse_accessor(stream)?;

            stream.ensure_symbol('=')?;

            let expr = expr::parse_expr_or_die(stream)?;
            let stmt = LetStatement {
                name,
                accessor,
                expr,
            };

            stream.ensure_symbol(';')?;
            Ok(Statement::Let(stmt))
        })
    });
}

fn parse_accessor(stream: &mut Stream) -> Result<Option<Expr>> {
    trace!(stream, "parse_accessor", {
        stream
            .consume_if_symbol('[')
            .map(|_t| {
                expr::parse_expr_or_die(stream)
                    .and_then(|expr| stream.ensure_symbol(']').and_then(|_t| Ok(expr)))
            })
            .transpose()
    });
}

fn parse_cond_expr(stream: &mut Stream) -> Result<Expr> {
    stream.ensure_symbol('(')?;

    let cond = expr::parse_expr_or_die(stream)?;

    stream.ensure_symbol(')')?;

    Ok(cond)
}

fn parse_block(stream: &mut Stream) -> Result<Statements> {
    stream.ensure_symbol('{')?;
    let statements = parse_statements(stream)?;
    stream.ensure_symbol('}')?;

    Ok(statements)
}

fn parse_if(stream: &mut Stream) -> Option<Result<Statement>> {
    trace!(stream, "parse_if", {
        stream.consume_if_keyword(Keyword::If).map(|_t| {
            let cond = parse_cond_expr(stream)?;
            let statements = parse_block(stream)?;

            let else_branch = stream
                .consume_if_keyword(Keyword::Else)
                .map(|_t| parse_block(stream))
                .transpose()?;

            let stmt = IfStatement {
                cond,
                statements,
                else_branch,
            };
            Ok(Statement::If(stmt))
        })
    });
}

fn parse_while(stream: &mut Stream) -> Option<Result<Statement>> {
    trace!(stream, "parse_while", {
        stream.consume_if_keyword(Keyword::While).map(|_t| {
            let cond = parse_cond_expr(stream)?;
            let statements = parse_block(stream)?;

            let stmt = WhileStatement { cond, statements };
            Ok(Statement::While(stmt))
        })
    });
}

fn parse_do(stream: &mut Stream) -> Option<Result<Statement>> {
    trace!(stream, "parse_do", {
        stream.consume_if_keyword(Keyword::Do).map(|_t| {
            let call = expr::parse_subroutine_call(stream)?;

            stream.ensure_symbol(';')?;

            let stmt = DoStatement { call };
            Ok(Statement::Do(stmt))
        })
    });
}

fn parse_return(stream: &mut Stream) -> Option<Result<Statement>> {
    trace!(stream, "parse_return", {
        stream.consume_if_keyword(Keyword::Return).map(|_t| {
            let expr = expr::parse_expr(stream).transpose()?;

            stream.ensure_symbol(';')?;

            let stmt = ReturnStatement { expr };
            Ok(Statement::Return(stmt))
        })
    });
}
