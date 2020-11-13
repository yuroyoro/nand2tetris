use super::expr::write_expr;
use super::symbols::Symbols;
use super::typedef::FuncDef;
use super::writer::Writer;
use super::*;
use crate::parser::ast::*;

use anyhow::{anyhow, Result};

pub fn write_function(
    w: &mut Writer,
    symbols: &mut Symbols,
    cls: &Class,
    sub: &SubroutineDec,
) -> Result<()> {
    match sub.modifier {
        SubroutineModifier::Constructor => write_subroutine_constructor(w, symbols, cls, sub),
        SubroutineModifier::Method => write_subroutine_method(w, symbols, cls, sub),
        SubroutineModifier::Function => write_subroutine_function(w, symbols, cls, sub),
    }
}

fn write_function_def(w: &mut Writer, sub: &SubroutineDec) -> Result<()> {
    let nlocals = sub.body.vars.iter().map(|var| var.names.len()).sum();

    w.function(&sub.name, nlocals)
}

pub fn write_subroutine_constructor(
    w: &mut Writer,
    symbols: &mut Symbols,
    cls: &Class,
    sub: &SubroutineDec,
) -> Result<()> {
    let mut symbols = symbols.constructor(cls, sub);

    // constructor
    write_function_def(w, sub)?;

    // alloc object and set this pointer
    let size: usize = cls.field_vars().iter().map(|var| var.names.len()).sum();
    w.push_constant(size as u16)?;
    w.call("Memory.alloc", 1)?;
    w.set_this()?;

    statement::write_statements(w, &mut symbols, &sub.body.statements)
}

pub fn write_subroutine_method(
    w: &mut Writer,
    symbols: &mut Symbols,
    cls: &Class,
    sub: &SubroutineDec,
) -> Result<()> {
    let mut symbols = symbols.method(cls, sub);

    // method
    write_function_def(w, sub)?;

    // set this pointer
    w.push_from_args(0)?;
    w.set_this()?;

    statement::write_statements(w, &mut symbols, &sub.body.statements)
}

pub fn write_subroutine_function(
    w: &mut Writer,
    symbols: &mut Symbols,
    cls: &Class,
    sub: &SubroutineDec,
) -> Result<()> {
    let mut symbols = symbols.function(cls, sub);

    // function
    write_function_def(w, sub)?;

    statement::write_statements(w, &mut symbols, &sub.body.statements)
}

pub fn write_call(w: &mut Writer, symbols: &mut Symbols, call: &SubroutineCall) -> Result<()> {
    // resolve reciever
    let reciever = extract_reciever_name(call);

    if let Some(sym) = symbols.lookup(&reciever) {
        let type_error = || {
            anyhow!(
                "{:?}: cloud not call method ({}) on primitive type {}",
                call.loc,
                sym.name,
                sym.typ.display(),
            )
        };

        // check reciever is not primitive type
        if !sym.typ.is_class() {
            return Err(type_error());
        }

        let clsname = sym.typ.extract_class().ok_or_else(type_error)?;
        let cls = symbols.lookup_type_or_die(&clsname, &call.loc)?;

        // check target class has method
        let func = cls.method(&call.name).ok_or_else(|| {
            anyhow!(
                "{:?}: could not found method {} on class {}",
                call.loc,
                &call.name,
                clsname
            )
        })?;

        // check method paramter count
        check_nargs(call, func)?;

        // TODO: check parameter's type

        let nargs = call.exprs.len() + 1;

        // set reciever to stack
        w.push_from(sym)?;

        // push arguments
        for expr in call.exprs.iter() {
            write_expr(w, symbols, expr)?;
        }

        let func_name = format!("{}.{}", clsname, &call.name);

        w.call(&func_name, nargs)
    } else {
        write_function_call(w, symbols, call, &reciever)
    }
}

fn extract_reciever_name(call: &SubroutineCall) -> String {
    if let Some(ref recv) = call.reciever {
        recv.to_string()
    } else {
        "this".to_string()
    }
}

fn write_function_call(
    w: &mut Writer,
    symbols: &mut Symbols,
    call: &SubroutineCall,
    clsname: &str,
) -> Result<()> {
    // lookup type
    let cls = symbols.lookup_type_or_die(clsname, &call.loc)?;

    // check target class has method
    let func = cls
        .function(&call.name)
        .or_else(|| cls.constructor(&call.name))
        .ok_or_else(|| {
            anyhow!(
                "{:?}: could not found functoin {} on class {}",
                call.loc,
                &call.name,
                clsname
            )
        })?;

    // check method paramter count
    check_nargs(call, func)?;

    // TODO: check parameter's type

    let nargs = call.exprs.len();

    // push arguments
    for expr in call.exprs.iter() {
        write_expr(w, symbols, expr)?;
    }

    let func_name = format!("{}.{}", clsname, &call.name);

    w.call(&func_name, nargs)
}

fn check_nargs(call: &SubroutineCall, func: &FuncDef) -> Result<()> {
    if call.exprs.len() != func.args.len() {
        return Err(anyhow!(
            "{:?}: '{}' takes {} argument but {} arguments were supplied",
            call.loc,
            &call.name,
            func.args.len(),
            call.exprs.len()
        ));
    }

    Ok(())
}
