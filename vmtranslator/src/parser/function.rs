use crate::parser::symbol::validate_symbol;
use crate::parser::{Command, Source};
use anyhow::{anyhow, Result};

pub fn parse(cmd: &str, current_function: &str, source: &Source, arg1: Option<&str>, arg2: Option<&str>) -> Option<Result<Command>> {
    match cmd {
        "function" => Some(parse_function(arg1, arg2, source.clone())),
        "call" => Some(parse_call(arg1, arg2, source.clone())),
        "return" => Some(parse_return(current_function, source.clone())),
        _ => None,
    }
}

fn parse_function(arg1: Option<&str>, arg2: Option<&str>, source: Source) -> Result<Command> {
    let name = parse_function_name(arg1, &source)?;
    let locals = parse_number(arg2, &source)?;

    Ok(Command::Function(name, locals, source))
}

fn parse_call(arg1: Option<&str>, arg2: Option<&str>, source: Source) -> Result<Command> {
    let name = parse_function_name(arg1, &source)?;
    let arity = parse_number(arg2, &source)?;

    Ok(Command::Call(name, arity, source))
}

fn parse_return(current_function: &str, source: Source) -> Result<Command> {
    if current_function == "" {
        Err(anyhow!("{:?} : return : current_function is emtpy", source))
    } else {
        Ok(Command::Return(source))
    }
}

pub fn parse_function_name(symbol: Option<&str>, source: &Source) -> Result<String> {
    symbol
        .ok_or(anyhow!("{:?} : expected function name but empty", &source))
        .and_then(|lbl| validate_symbol(lbl, source))
}

fn parse_number(arg2: Option<&str>, source: &Source) -> Result<i64> {
    let number = arg2.ok_or(anyhow!("{:?} : expected number but empty", &source))?;
    number
        .parse::<i64>()
        .map_err(|err| anyhow!("{:?} : invalid number: {}, {}", source, number, err))
}
