pub mod arithmetic;
pub mod flow;
pub mod function;
pub mod stack;
pub mod symbol;

use anyhow::{anyhow, Result};

pub struct ParseResult {
    pub commands: Vec<Command>,
    pub errors: Vec<anyhow::Error>,
    pub vm_name: String,
}

#[derive(Debug, Clone)]
pub enum Command {
    // arithmetic commands
    Add(Source),
    Sub(Source),
    Neg(Source),
    Eq(Source),
    Gt(Source),
    Lt(Source),
    And(Source),
    Or(Source),
    Not(Source),
    // memory access commands
    Push(Segment, i64, Source),
    Pop(Segment, i64, Source),
    // program flow commands
    Label(String, Source),
    Goto(String, Source),
    IfGoto(String, Source),
    // function commands
    Function(String, i64, Source),
    Call(String, i64, Source),
    Return(Source),
}

#[derive(Debug, Clone, PartialEq, enum_utils::FromStr)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub vm_name: String,
    pub line: usize,  // line number in source file
    pub code: String, // original source code
}

const COMMENT: &'static str = "//";

pub fn parse(content: &str, vm_name: &str) -> ParseResult {
    let sources = parse_lines(content, vm_name);
    parse_sources(sources, vm_name)
}

fn parse_lines(contents: &str, vm_name: &str) -> Vec<Source> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(line, content)| build_source(content, line, vm_name))
        .collect()
}

fn drop_whitespaces(content: &str) -> String {
    content
        .chars()
        .skip_while(|c| c.is_whitespace())
        .collect::<String>()
}

fn build_source(content: &str, line: usize, vm_name: &str) -> Option<Source> {
    // drop whitespace
    let code = drop_whitespaces(content);
    let code = code.splitn(2, COMMENT).next().unwrap_or("");

    if code.len() > 0 {
        Some(Source {
            vm_name: vm_name.to_string(),
            line: line + 1,
            code: String::from(code),
        })
    } else {
        None
    }
}

fn parse_sources(sources: Vec<Source>, vm_name: &str) -> ParseResult {
    let vm_name = vm_name.to_string();
    let mut commands = Vec::new();
    let mut errors = Vec::new();
    let mut current_function = String::new();

    sources.into_iter().for_each(|source| {
        match instrument(source, &current_function) {
            Ok(Command::Function(name, nlocal, source)) => {
                // update current function name
                current_function.clear();
                current_function.push_str(&name);

                commands.push(Command::Function(name, nlocal, source))
            }
            Ok(cmd) => commands.push(cmd),
            Err(error) => errors.push(error),
        };
    });

    ParseResult {
        commands,
        errors,
        vm_name,
    }
}

fn instrument(source: Source, current_function: &str) -> Result<Command> {
    let code = source.code.clone();
    let mut iter = code.split_ascii_whitespace();
    let cmd = iter
        .next()
        .ok_or(anyhow!("{:?} : unexpected blank line", &source))?;

    let arg1 = iter.next();
    let arg2 = iter.next();

    arithmetic::parse(&cmd, current_function, &source, arg1, arg2)
        .or_else(|| stack::parse(&cmd, current_function, &source, arg1, arg2))
        .or_else(|| flow::parse(&cmd, current_function, &source, arg1, arg2))
        .unwrap_or(Err(anyhow!(
            "{:?} : unexpected vm command : {}",
            &source,
            &cmd
        )))
}
