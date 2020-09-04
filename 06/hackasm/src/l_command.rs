use super::parser::*;
use super::symbols::*;

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct LCommand {
    pub symbol: Symbol,
    pub addr:   usize,
    source:     Source,
}

pub fn parse(addr: usize, source: Source) -> Result<LCommand> {
    if !source.code.ends_with(")") {
        return Err(anyhow!("{:?} : expected `)`", source));
    }
    let name = source.code.get(1..(source.code.len() - 1)).unwrap();
    let symbol = Symbol::new(String::from(name), addr);

    Ok(LCommand {
        symbol,
        addr: addr,
        source,
    })
}
