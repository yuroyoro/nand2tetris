use super::function::write_call;
use super::symbols::Symbols;
use super::writer::Writer;
use super::*;
use crate::parser::ast::*;
use crate::token::Location;

use anyhow::Result;

pub fn write_expr(w: &mut Writer, symbols: &mut Symbols, expr: &Expr) -> Result<()> {
    // lhs
    write_term(w, symbols, &expr.lhs, &expr.loc)?;

    // rhs
    if let Some((op, ref rexpr)) = expr.rhs.as_ref() {
        write_expr(w, symbols, rexpr)?;
        write_op(w, op)?;
    }

    Ok(())
}

fn write_op(w: &mut Writer, op: &Op) -> Result<()> {
    match op {
        Op::Add => w.arithmetic(Command::Add),
        Op::Sub => w.arithmetic(Command::Sub),
        Op::Mul => w.call("Math.multiply", 2),
        Op::Div => w.call("Math.divide", 2),
        Op::And => w.arithmetic(Command::And),
        Op::Or => w.arithmetic(Command::Or),
        Op::Lt => w.arithmetic(Command::Lt),
        Op::Gt => w.arithmetic(Command::Gt),
        Op::Eq => w.arithmetic(Command::Eq),
    }
}

pub fn write_term(
    w: &mut Writer,
    symbols: &mut Symbols,
    term: &Term,
    loc: &Location,
) -> Result<()> {
    match term {
        Term::Integer(n) => write_integer(w, *n),
        Term::Str(s) => write_string(w, &s),
        Term::Keyword(ref kwd) => write_keyword(w, kwd),
        Term::Var(ident) => write_var(w, symbols, ident, None, loc),
        Term::IndexAccess(ident, ref expr) => write_var(w, symbols, ident, Some(expr), loc),
        Term::Call(ref call) => write_call(w, symbols, call),
        Term::Expr(ref expr) => write_expr(w, symbols, expr),
        Term::Unary(ref unaryop, ref term) => write_unary(w, symbols, unaryop, term, loc),
    }
}

fn write_integer(w: &mut Writer, n: u16) -> Result<()> {
    w.push_constant(n)
}

fn write_string(w: &mut Writer, s: &str) -> Result<()> {
    // push string length to pass String.new
    w.push_constant(s.len() as u16)?;

    // call String.new 1
    w.call("String.new", 1)?;

    for c in s.bytes() {
        w.push_constant(c as u16)?;
        w.call("String.appendChar", 2)?;
    }

    Ok(())
}

fn write_keyword(w: &mut Writer, kwd: &KeywordConst) -> Result<()> {
    match kwd {
        KeywordConst::True => {
            w.push_constant(1)?;
            w.arithmetic(Command::Neg)
        }
        KeywordConst::False => w.push_constant(0),
        KeywordConst::Null => w.push_constant(0),
        KeywordConst::This => w.get_this(),
    }
}

pub fn write_var(
    w: &mut Writer,
    symbols: &mut Symbols,
    ident: &str,
    accessor: Option<&Expr>,
    loc: &Location,
) -> Result<()> {
    // eval 'ident' or 'ident[accessor]'

    // base addr of name
    let base = symbols.lookup_or_die(ident, loc)?;
    w.push_from(base)?;

    // calculate addr if accesor is exist (base = &name + accessor)
    if let Some(accessor) = accessor {
        write_expr(w, symbols, accessor)?; // eval accessor expr
        w.arithmetic(Command::Add)?; // calculate &name + accessor and push the result

        // then calculated addr is pushed to stack top
        w.set_that()?; // set calculated addr to THAT segment base
        w.push_from_that(0)?; // read THAT 0
    }

    Ok(())
}

fn write_unary(
    w: &mut Writer,
    symbols: &mut Symbols,
    unaryop: &UnaryOp,
    term: &Term,
    loc: &Location,
) -> Result<()> {
    write_term(w, symbols, term, loc)?;
    match unaryop {
        UnaryOp::Minus => w.arithmetic(Command::Neg),
        UnaryOp::Not => w.arithmetic(Command::Not),
    }
}
