use crate::parser::symbol::validate_symbol;
use crate::parser::{Command, Source};
use anyhow::{anyhow, Result};

pub fn parse(cmd: &str, current_function: &str, source: &Source, arg1: Option<&str>, _arg2: Option<&str>) -> Option<Result<Command>> {
    match cmd {
        "label" => Some(parse_label_cmd(arg1, current_function, source.clone())),
        "goto" => Some(parse_goto(arg1, current_function, source.clone())),
        "if-goto" => Some(parse_if_goto(arg1, current_function, source.clone())),
        _ => None,
    }
}

fn parse_label_cmd(label: Option<&str>, current_function: &str, source: Source) -> Result<Command> {
    let label = parse_label(label, current_function, &source)?;
    Ok(Command::Label(label, source))
}

fn parse_goto(label: Option<&str>, current_function: &str, source: Source) -> Result<Command> {
    let label = parse_label(label, current_function, &source)?;
    Ok(Command::Goto(label, source))
}

fn parse_if_goto(label: Option<&str>, current_function: &str, source: Source) -> Result<Command> {
    let label = parse_label(label, current_function, &source)?;
    Ok(Command::IfGoto(label, source))
}

pub fn parse_label(symbol: Option<&str>, current_function: &str, source: &Source) -> Result<String> {
    symbol
        .ok_or(anyhow!("{:?} : expected symbol but empty", &source))
        .and_then(|lbl| validate_symbol(lbl, source))
        .map(|lbl| format!("{}${}", current_function, lbl))
}
