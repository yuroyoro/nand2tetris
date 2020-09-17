use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum Command {
    // arithmetic commands
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    // memory access commands
    Push(Segment, i64),
    Pop(Segment, i64),
    // program flow commands
    // Label(String),
    // Goto(String),
    // IfGoto(String),
    // function commands
    // Function(String, i64),
    // Call(String, i64),
    // Return
}

#[derive(Debug, Clone, enum_utils::FromStr)]
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
    pub line: usize,  // line number in source file
    pub code: String, // original source code
}

pub type ParseResult = (Vec<Command>, Vec<anyhow::Error>);

const COMMENT: &'static str = "//";

pub fn parse(content: &str) -> ParseResult {
    let sources = parse_lines(content);
    parse_sources(sources)
}

fn parse_lines(contents: &str) -> Vec<Source> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(line, content)| build_source(content, line))
        .collect()
}

fn drop_whitespaces(content: &str) -> String {
    content
        .chars()
        .skip_while(|c| c.is_whitespace())
        .collect::<String>()
}

fn build_source(content: &str, line: usize) -> Option<Source> {
    // drop whitespace
    let code = drop_whitespaces(content);
    let code = code.splitn(2, COMMENT).next().unwrap_or("");

    if code.len() > 0 {
        Some(Source {
            line: line + 1,
            code: String::from(code),
        })
    } else {
        None
    }
}

fn parse_sources(sources: Vec<Source>) -> ParseResult {
    let mut commands = Vec::new();
    let mut errors = Vec::new();

    sources.into_iter().for_each(|source| {
        match instrument(source) {
            Ok(cmd) => commands.push(cmd),
            Err(error) => errors.push(error),
        };
    });

    (commands, errors)
}

fn instrument(source: Source) -> Result<Command> {
    let mut iter = source.code.split_ascii_whitespace();
    let cmd = iter
        .next()
        .ok_or(anyhow!("{:?} : unexpected blank line", &source))?;

    parse_arithmetic_command(&cmd)
        .or_else(|| parse_memmory_command(&cmd, &source, &mut iter))
        .unwrap_or(Err(anyhow!(
            "{:?} : unexpected vm command : {}",
            &source,
            &cmd
        )))
}

fn parse_arithmetic_command(cmd: &str) -> Option<Result<Command>> {
    match cmd {
        "add" => Some(Ok(Command::Add)),
        "sub" => Some(Ok(Command::Sub)),
        "neg" => Some(Ok(Command::Neg)),
        "eq" => Some(Ok(Command::Eq)),
        "gt" => Some(Ok(Command::Gt)),
        "lt" => Some(Ok(Command::Lt)),
        "and" => Some(Ok(Command::And)),
        "or" => Some(Ok(Command::Or)),
        "not" => Some(Ok(Command::Not)),
        _ => None,
    }
}

fn parse_memmory_command(
    cmd: &str,
    source: &Source,
    iter: &mut std::str::SplitAsciiWhitespace,
) -> Option<Result<Command>> {
    let segment = iter
        .next()
        .map(|seg| parse_segment(seg))
        .unwrap_or(Err(anyhow!("{:?} : expected segment but empty", &source)));

    let segment = match segment {
        Ok(seg) => seg,
        Err(err) => return Some(Err(err)),
    };

    let index = iter
        .next()
        .map(|idx| parse_index(idx))
        .unwrap_or(Err(anyhow!("{:?} : expected segment but empty", &source)));

    let index = match index {
        Ok(idx) => idx,
        Err(err) => return Some(Err(err)),
    };

    match cmd {
        "push" => Some(Ok(Command::Push(segment, index))),
        "pop" => Some(Ok(Command::Pop(segment, index))),
        _ => None,
    }
}

fn to_uppercase_first_char(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    format!("{}{}", head.to_uppercase(), tail)
}

fn parse_segment(segment: &str) -> Result<Segment> {
    let segment = to_uppercase_first_char(segment);

    segment
        .parse::<Segment>()
        .map_err(|()| anyhow!("invalid segment: {}", segment))
}

fn parse_index(index: &str) -> Result<i64> {
    index
        .parse::<i64>()
        .map_err(|err| anyhow!("invalid index: {}, {}", index, err))
}
