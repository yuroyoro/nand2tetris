use crate::parser::{Command, Source};
use anyhow::Result;

pub fn parse(cmd: &str, _current_function: &str, source: &Source, _arg1: Option<&str>, _arg2: Option<&str>) -> Option<Result<Command>> {
    match cmd {
        "add" => Some(Ok(Command::Add(source.clone()))),
        "sub" => Some(Ok(Command::Sub(source.clone()))),
        "neg" => Some(Ok(Command::Neg(source.clone()))),
        "eq" => Some(Ok(Command::Eq(source.clone()))),
        "gt" => Some(Ok(Command::Gt(source.clone()))),
        "lt" => Some(Ok(Command::Lt(source.clone()))),
        "and" => Some(Ok(Command::And(source.clone()))),
        "or" => Some(Ok(Command::Or(source.clone()))),
        "not" => Some(Ok(Command::Not(source.clone()))),
        _ => None,
    }
}
