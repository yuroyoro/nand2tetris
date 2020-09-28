use crate::parser::{Command, Segment, Source};
use anyhow::{anyhow, Result};

pub fn parse(cmd: &str, _current_function: &str, source: &Source, arg1: Option<&str>, arg2: Option<&str>) -> Option<Result<Command>> {
    match cmd {
        "push" => Some(parse_push(arg1, arg2, source.clone())),
        "pop" => Some(parse_pop(arg1, arg2, source.clone())),
        _ => None,
    }
}

fn parse_push(arg1: Option<&str>, arg2: Option<&str>, source: Source) -> Result<Command> {
    let segment = parse_segment(arg1, &source)?;
    let index = parse_index(arg2, &segment, &source)?;
    Ok(Command::Push(segment, index, source))
}

fn parse_pop(arg1: Option<&str>, arg2: Option<&str>, source: Source) -> Result<Command> {
    let segment = parse_segment(arg1, &source)?;
    if segment == Segment::Constant {
        return Err(anyhow!("{:?} : can not pop to constant segment", &source));
    }

    let index = parse_index(arg2, &segment, &source)?;
    Ok(Command::Pop(segment, index, source))
}

fn to_uppercase_first_char(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    format!("{}{}", head.to_uppercase(), tail)
}

fn parse_segment(segment: Option<&str>, source: &Source) -> Result<Segment> {
    let segment = segment.ok_or(anyhow!("{:?} : expected segment but empty", &source))?;
    let segment = to_uppercase_first_char(segment);

    segment.parse::<Segment>().map_err(|()| anyhow!("{:?} : invalid segment: {}", source, segment))
}

fn parse_index(index: Option<&str>, segment: &Segment, source: &Source) -> Result<i64> {
    let index = index.ok_or(anyhow!("{:?} : expected index but empty", &source))?;
    index
        .parse::<i64>()
        .map_err(|err| anyhow!("{:?} : invalid index: {}, {}", source, index, err))
        .and_then(|idx| validate_index(segment, idx, source))
}

fn validate_index(segment: &Segment, index: i64, source: &Source) -> Result<i64> {
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
