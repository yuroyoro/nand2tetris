use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::parser::*;
use ast::*;

use anyhow::{anyhow, Context, Result};

struct Writer {
    path: PathBuf,
    file: File,
    depth: usize,
}

impl Writer {
    fn new(path: PathBuf) -> Result<Writer> {
        let file = File::create(path.clone())?;

        Ok(Writer {
            path: path,
            file: file,
            depth: 0,
        })
    }

    fn open<F>(&mut self, tag: &str, f: F) -> Result<()>
    where
        F: FnOnce(&mut Writer) -> Result<()>,
    {
        self.write(format!("<{}>", tag).as_str())?;
        self.depth += 1;
        f(self)?;
        self.depth -= 1;
        self.write(format!("</{}>", tag).as_str())
    }

    fn tag(&mut self, tag: &str, s: &str) -> Result<()> {
        let spacer = "  ".repeat(self.depth);
        writeln!(self.file, "{}<{}> {} </{}>", spacer, tag, s, tag)
            .with_context(|| format!("failed to write xml file : {}", self.path.display()))
    }

    fn keyword(&mut self, s: &str) -> Result<()> {
        self.tag("keyword", s)
    }

    fn symbol(&mut self, s: &str) -> Result<()> {
        self.tag("symbol", s)
    }

    fn identifier(&mut self, s: &str) -> Result<()> {
        self.tag("identifier", s)
    }

    fn typ(&mut self, ty: &Type) -> Result<()> {
        match ty {
            Type::Int | Type::Char | Type::Boolean => self.keyword(&ty.display()),
            Type::Class(name) => self.identifier(&name),
        }
    }

    fn write(&mut self, s: &str) -> Result<()> {
        let spacer = "  ".repeat(self.depth);
        writeln!(self.file, "{}{}", spacer, s)
            .with_context(|| format!("failed to write xml file : {}", self.path.display()))
    }

    fn flush(&mut self) -> Result<()> {
        self.file
            .flush()
            .with_context(|| format!("failed to flush xml file : {}", self.path.display()))
    }
}

pub fn write_asts(asts: ASTs) -> Result<()> {
    let path = asts.source.ast_xml_filename()?;
    let mut w = Writer::new(path)?;

    write_class(&mut w, &asts.class)?;
    w.flush()
}

fn write_class(f: &mut Writer, cls: &Class) -> Result<()> {
    f.open("class", |f| {
        f.keyword("class")?;
        f.identifier(&cls.name)?;
        f.symbol("{")?;

        cls.vars
            .iter()
            .map(|var| write_class_var_dec(f, var))
            .collect::<Result<()>>()?;

        cls.subroutines
            .iter()
            .map(|sub| write_subroutine_dec(f, sub))
            .collect::<Result<()>>()?;

        f.symbol("}")
    })
}

fn write_class_var_dec(f: &mut Writer, var: &ClassVarDec) -> Result<()> {
    f.open("classVarDec", |f| {
        f.keyword(&var.modifier.display())?;
        f.typ(&var.typ)?;

        let (head, rest) = var
            .names
            .split_first()
            .ok_or_else(|| anyhow!("write_class_var_dec: empty vars"))?;

        f.identifier(head)?;

        rest.iter()
            .map(|name| f.symbol(",").and(f.identifier(&name)))
            .collect::<Result<()>>()?;

        f.symbol(";")
    })
}

fn write_subroutine_dec(f: &mut Writer, sub: &SubroutineDec) -> Result<()> {
    f.open("subroutineDec", |f| {
        f.keyword(&sub.modifier.display())?;
        match sub.typ {
            ReturnType::Void => f.keyword("void"),
            ReturnType::Type(ref ty) => f.typ(ty),
        }?;

        f.identifier(&sub.name)?;

        write_parameters(f, &sub.parameters)?;

        write_subroutine_body(f, &sub.body)
    })
}

fn write_parameters(f: &mut Writer, params: &Vec<ParameterDec>) -> Result<()> {
    f.symbol("(")?;

    f.open("parameterList", |f| {
        params
            .split_first()
            .map(|(head, rest)| {
                write_parameter(f, head)?;
                rest.iter()
                    .map(|param| f.symbol(",").and(write_parameter(f, param)))
                    .collect::<Result<()>>()
            })
            .unwrap_or(Ok(()))
    })?;

    f.symbol(")")
}

fn write_parameter(f: &mut Writer, param: &ParameterDec) -> Result<()> {
    f.typ(&param.typ)?;
    f.identifier(&param.name)
}

fn write_subroutine_body(f: &mut Writer, body: &SubroutineBody) -> Result<()> {
    f.open("subroutineBody", |f| {
        f.symbol("{")?;

        body.vars
            .iter()
            .map(|var| write_var_dec(f, var))
            .collect::<Result<()>>()?;

        write_statements(f, &body.statements)?;

        f.symbol("}")
    })
}

fn write_var_dec(f: &mut Writer, var: &VarDec) -> Result<()> {
    f.open("varDec", |f| {
        f.keyword("var")?;
        f.typ(&var.typ)?;

        let (head, rest) = var
            .names
            .split_first()
            .ok_or_else(|| anyhow!("write_var_dec: empty vars"))?;

        f.identifier(head)?;

        rest.iter()
            .map(|name| f.symbol(",").and(f.identifier(&name)))
            .collect::<Result<()>>()?;

        f.symbol(";")
    })
}
fn write_statements(f: &mut Writer, statements: &Statements) -> Result<()> {
    f.open("statements", |f| {
        statements
            .statements
            .iter()
            .map(|stmt| write_statement(f, stmt))
            .collect::<Result<()>>()
    })
}

fn write_statement(f: &mut Writer, stmt: &Statement) -> Result<()> {
    match stmt {
        Statement::Let(stmt) => write_let_stmt(f, stmt),
        Statement::If(stmt) => write_if_stmt(f, stmt),
        Statement::While(stmt) => write_while_stmt(f, stmt),
        Statement::Do(stmt) => write_do_stmt(f, stmt),
        Statement::Return(stmt) => write_return_stmt(f, stmt),
    }
}

fn write_let_stmt(f: &mut Writer, stmt: &LetStatement) -> Result<()> {
    f.open("letStatement", |f| {
        f.keyword("let")?;

        f.identifier(&stmt.name)?;
        stmt.accessor
            .as_ref()
            .map(|expr| {
                f.symbol("[")?;

                write_expr(f, &expr)?;

                f.symbol("]")
            })
            .unwrap_or(Ok(()))?;

        f.symbol("=")?;

        write_expr(f, &stmt.expr)?;

        f.symbol(";")
    })
}

fn write_if_stmt(f: &mut Writer, stmt: &IfStatement) -> Result<()> {
    f.open("ifStatement", |f| {
        f.keyword("if")?;

        f.symbol("(")?;

        write_expr(f, &stmt.cond)?;

        f.symbol(")")?;
        f.symbol("{")?;

        write_statements(f, &stmt.statements)?;

        stmt.else_branch
            .as_ref()
            .map(|els| {
                f.symbol("}")?;
                f.keyword("else")?;
                f.symbol("{")?;
                write_statements(f, &els)
            })
            .unwrap_or(Ok(()))?;

        f.symbol("}")
    })
}

fn write_while_stmt(f: &mut Writer, stmt: &WhileStatement) -> Result<()> {
    f.open("whileStatement", |f| {
        f.keyword("while")?;

        f.symbol("(")?;

        write_expr(f, &stmt.cond)?;

        f.symbol(")")?;
        f.symbol("{")?;

        write_statements(f, &stmt.statements)?;

        f.symbol("}")
    })
}

fn write_do_stmt(f: &mut Writer, stmt: &DoStatement) -> Result<()> {
    f.open("doStatement", |f| {
        f.keyword("do")?;
        write_call(f, &stmt.call)?;
        f.symbol(";")
    })
}

fn write_return_stmt(f: &mut Writer, stmt: &ReturnStatement) -> Result<()> {
    f.open("returnStatement", |f| {
        f.keyword("return")?;

        stmt.expr
            .as_ref()
            .map(|expr| write_expr(f, &expr))
            .unwrap_or(Ok(()))?;
        f.symbol(";")
    })
}

fn write_expr(f: &mut Writer, expr: &Expr) -> Result<()> {
    f.open("expression", |f| {
        write_term(f, &expr.lhs)?;

        let mut rhs = &expr.rhs;

        while let Some((op, e)) = rhs.as_ref() {
            f.symbol(&op.display())?;
            write_term(f, &e.lhs)?;
            rhs = &e.rhs;
        }

        Ok(())
    })
}

/** write expr tree with operator priority
fn write_expr(f: &mut Writer, expr: &Expr) -> Result<()> {
    f.open("expression", |f| {
        write_term(f, &expr.lhs)?;

        expr.rhs
            .as_ref()
            .as_ref()
            .map(|(op, e)| {
                f.symbol(&op.display())?;
                write_expr(f, &e)
            })
            .unwrap_or(Ok(()))
    })
}
*/

fn write_term(f: &mut Writer, term: &Term) -> Result<()> {
    f.open("term", |f| match term {
        Term::Integer(n) => f.tag("integerConstant", &n.to_string()),
        Term::Str(s) => f.tag("stringConstant", &s),
        Term::Keyword(kwd) => f.keyword(&kwd.display()),
        Term::Var(ident) => f.identifier(ident),
        Term::IndexAccess(ident, expr) => {
            f.identifier(ident)?;
            f.symbol("[")?;
            write_expr(f, expr)?;
            f.symbol("]")
        }
        Term::Call(call) => write_call(f, call),
        Term::Expr(expr) => {
            f.symbol("(")?;
            write_expr(f, expr)?;
            f.symbol(")")
        }
        Term::Unary(unaryop, term) => {
            f.symbol(&unaryop.display())?;
            write_term(f, term)
        }
    })
}

fn write_call(f: &mut Writer, call: &SubroutineCall) -> Result<()> {
    call.reciever
        .as_ref()
        .map(|recv| {
            f.identifier(&recv)?;
            f.symbol(".")
        })
        .unwrap_or(Ok(()))?;

    f.identifier(&call.name)?;
    f.symbol("(")?;

    f.open("expressionList", |f| {
        call.exprs
            .split_first()
            .map(|(head, rest)| {
                write_expr(f, head)?;
                rest.iter()
                    .map(|param| f.symbol(",").and(write_expr(f, param)))
                    .collect::<Result<()>>()
            })
            .unwrap_or(Ok(()))
    })?;

    f.symbol(")")
}
