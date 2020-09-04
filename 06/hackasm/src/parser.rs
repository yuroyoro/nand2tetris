// parser
use super::a_command::*;
use super::c_command::*;
use super::l_command::*;
use super::symbols::Symbols;

use anyhow::Result;

#[derive(Debug)]
pub enum Node {
    A(ACommand),
    C(CCommand),
    L(LCommand),
}

#[derive(Debug, Clone)]
pub struct Source {
    pub line: usize,  // line number in source file
    pub code: String, // original source code
}

pub type ParseResult = (Vec<Node>, Vec<anyhow::Error>);

const COMMENT: &'static str = "//";

pub fn parse(contents: &str, symbols: &mut Symbols) -> ParseResult {
    let sources = parse_lines(contents);
    parse_sources(sources, symbols)
}

fn parse_lines(contents: &str) -> Vec<Source> {
    contents
        .lines()
        .enumerate()
        .filter_map(|(line, content)| {
            // drop whitespace
            let code = drop_whitespaces(content);
            let (code, _comment) = split_code(&code, COMMENT, false);

            code.filter(|c| c.len() > 0).map(|c| Source {
                line: line + 1,
                code: truncate_whitespaces(c),
            })
        })
        .collect()
}

fn drop_whitespaces(content: &str) -> String {
    content
        .chars()
        .skip_while(|c| c.is_whitespace())
        .collect::<String>()
}

fn parse_sources(sources: Vec<Source>, symbols: &mut Symbols) -> ParseResult {
    let mut nodes = Vec::new();
    let mut errors = Vec::new();

    let mut addr = 0;

    sources.into_iter().for_each(|source| {
        match instrument(source, addr) {
            Ok(Node::L(l)) => {
                // TODO: check duplicate symbol definition
                symbols.add(l.symbol.clone());
                nodes.push(Node::L(l))
            }
            Ok(node) => {
                nodes.push(node);
                addr += 1;
            }
            Err(error) => errors.push(error),
        };
    });

    (nodes, errors)
}

fn instrument(source: Source, addr: usize) -> Result<Node> {
    // A-Command @foo
    if source.code.starts_with("@") {
        return super::a_command::parse(addr, source).map(|a| Node::A(a));
    }

    // L-Command (LABEL)
    if source.code.starts_with("(") {
        return super::l_command::parse(addr, source).map(|l| Node::L(l));
    }

    // C-Command dest=comp;jmp
    super::c_command::parse(addr, source).map(|c| Node::C(c))
}

fn truncate_whitespaces(code: &str) -> String {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
}

pub fn split_code<'a>(
    code: &'a str,
    pat: &str,
    reverse: bool,
) -> (Option<&'a str>, Option<&'a str>) {
    let mut split = code.splitn(2, pat);
    let rhs = split.next();
    let lhs = split.next();

    if lhs.is_some() {
        (rhs, lhs)
    } else {
        if reverse {
            (None, rhs)
        } else {
            (rhs, None)
        }
    }
}
