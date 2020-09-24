use anyhow::{anyhow, Result};

pub struct ParseResult {
    pub commands: Vec<Command>,
    pub errors: Vec<anyhow::Error>,
    pub vm_name: String,
}

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

    sources.into_iter().for_each(|source| {
        match instrument(source) {
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
        .map(|seg| parse_segment(&source, seg))
        .unwrap_or(Err(anyhow!("{:?} : expected segment but empty", &source)));

    let segment = match segment {
        Ok(seg) => seg,
        Err(err) => return Some(Err(err)),
    };

    let index = iter
        .next()
        .map(|idx| parse_index(&source, &segment, idx))
        .unwrap_or(Err(anyhow!("{:?} : expected segment but empty", &source)));

    let index = match index {
        Ok(idx) => idx,
        Err(err) => return Some(Err(err)),
    };

    match cmd {
        "push" => Some(Ok(Command::Push(segment, index))),
        "pop" => {
            if segment == Segment::Constant {
                Some(Err(anyhow!(
                    "{:?} : can not pop to constant segment",
                    &source
                )))
            } else {
                Some(Ok(Command::Pop(segment, index)))
            }
        }
        _ => None,
    }
}

fn to_uppercase_first_char(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    format!("{}{}", head.to_uppercase(), tail)
}

fn parse_segment(source: &Source, segment: &str) -> Result<Segment> {
    let segment = to_uppercase_first_char(segment);

    segment
        .parse::<Segment>()
        .map_err(|()| anyhow!("{:?} : invalid segment: {}", source, segment))
}

fn parse_index(source: &Source, segment: &Segment, index: &str) -> Result<i64> {
    index
        .parse::<i64>()
        .map_err(|err| anyhow!("{:?} : invalid index: {}, {}", source, index, err))
        .and_then(|idx| validate_index(source, segment, idx))
}

fn validate_index(source: &Source, segment: &Segment, index: i64) -> Result<i64> {
    if index < 0 && *segment != Segment::Constant {
        return Err(anyhow!(
            "{:?} : illegal segment index, index must be zero or poisitive : {:?}, {}",
            &source,
            segment,
            index
        ));
    }

    if index > 2 && *segment == Segment::Pointer {
        return Err(anyhow!(
            "{:?} : illegal segment index, pointer index must be less than 2 : {:?}, {}",
            &source,
            segment,
            index
        ));
    }

    if index > 7 && *segment == Segment::Temp {
        return Err(anyhow!(
            "{:?} : illegal segment index, temp index must be less than 7 : {:?}, {}",
            &source,
            segment,
            index
        ));
    }

    if index > 255 && *segment == Segment::Static {
        return Err(anyhow!(
            "{:?} : illegal segment index, static index must be less than 255 : {:?}, {}",
            &source,
            segment,
            index
        ));
    }

    Ok(index)
}
