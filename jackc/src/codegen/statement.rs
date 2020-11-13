use super::expr::write_expr;
use super::function::write_call;
use super::symbols::Symbols;
use super::writer::Writer;
use super::*;
use crate::parser::ast::*;

use anyhow::Result;

pub fn write_statements(w: &mut Writer, symbols: &mut Symbols, stmts: &Statements) -> Result<()> {
    stmts
        .statements
        .iter()
        .map(|stmt| write_statement(w, symbols, stmt))
        .collect::<Result<()>>()
}

fn write_statement(w: &mut Writer, symbols: &mut Symbols, stmt: &Statement) -> Result<()> {
    match stmt {
        Statement::Let(stmt) => write_let_stmt(w, symbols, stmt),
        Statement::If(stmt) => write_if_stmt(w, symbols, stmt),
        Statement::While(stmt) => write_while_stmt(w, symbols, stmt),
        Statement::Do(stmt) => write_do_stmt(w, symbols, stmt),
        Statement::Return(stmt) => write_return_stmt(w, symbols, stmt),
    }
}

fn write_let_stmt(w: &mut Writer, symbols: &mut Symbols, stmt: &LetStatement) -> Result<()> {
    // let name[accessor] = expr;

    // eval rhs expr
    write_expr(w, symbols, &stmt.expr)?;

    // assign result to lhs
    let lhs = symbols.lookup_or_die(&stmt.name, &stmt.loc)?;

    if let Some(accessor) = stmt.accessor.as_ref() {
        w.push_from(lhs)?; // base addr
        write_expr(w, symbols, accessor)?; // eval accessor expr
        w.arithmetic(Command::Add)?; // calculate &name + accessor and push the result

        // then calculated addr is pushed to stack top
        // set addr to that
        w.set_that()?;
        // assign lhs value to that
        w.pop_to_that(0)
    } else {
        w.pop_to(lhs)
    }
}

fn write_if_stmt(w: &mut Writer, symbols: &mut Symbols, stmt: &IfStatement) -> Result<()> {
    // if (cond) { stmts } else { stmts }

    // eval cond expr
    write_cond_expr(w, symbols, &stmt.cond)?;

    let lbl_false = symbols.label("IF_FALSE");
    let lbl_end = symbols.label("IF_END");

    // if-goto IF_FALSE_0
    w.if_goto(&lbl_false)?;

    // eval true blanch statement
    write_statements(w, symbols, &stmt.statements)?;

    // goto IF_END_0
    w.goto(&lbl_end)?;

    // label IF_FALSE_0
    w.label(&lbl_false)?;

    // eval else branch statements (if exisits)
    if let Some(stmt) = stmt.else_branch.as_ref() {
        write_statements(w, symbols, &stmt)?;
    }

    // label IF_FALSE_0
    w.label(&lbl_end)
}

fn write_while_stmt(w: &mut Writer, symbols: &mut Symbols, stmt: &WhileStatement) -> Result<()> {
    // while (cond) { stmts }

    let lbl_start = symbols.label("WHILE_START");
    let lbl_end = symbols.label("WHILE_END");

    // label WHILE_START_0
    w.label(&lbl_start)?;

    // eval cond expr
    write_cond_expr(w, symbols, &stmt.cond)?;

    // goto WHILE_END
    w.if_goto(&lbl_end)?;

    // eval true blanch statement
    write_statements(w, symbols, &stmt.statements)?;

    // goto WHILE_START_0
    w.goto(&lbl_start)?;

    // label WHILE_END_0
    w.label(&lbl_end)
}

fn write_do_stmt(w: &mut Writer, symbols: &mut Symbols, stmt: &DoStatement) -> Result<()> {
    // eval call
    write_call(w, symbols, &stmt.call)?;

    // pop return value on statck top
    w.pop(Segment::Temp, 0)
}

fn write_return_stmt(w: &mut Writer, symbols: &mut Symbols, stmt: &ReturnStatement) -> Result<()> {
    // eval expr
    if let Some(expr) = stmt.expr.as_ref() {
        write_expr(w, symbols, expr)?;
    }

    w.return_cmd()
}

fn write_cond_expr(w: &mut Writer, symbols: &mut Symbols, expr: &Expr) -> Result<()> {
    write_expr(w, symbols, expr)?;
    w.arithmetic(Command::Not)
}
