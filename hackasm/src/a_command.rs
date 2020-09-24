use super::parser::*;
use super::symbols::*;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct ACommand {
    pub symbol_name: Option<String>,
    pub addr:        usize,
    pub value:       i64,
    source:          Source,
}

impl ACommand {
    pub fn new(addr: usize, sym: String, value: i64, source: Source) -> ACommand {
        ACommand {
            symbol_name: Some(sym),
            addr:        addr,
            value:       value,
            source:      source,
        }
    }

    pub fn new_with_value(addr: usize, value: i64, source: Source) -> ACommand {
        ACommand {
            symbol_name: None,
            addr:        addr,
            value:       value,
            source:      source,
        }
    }

    pub fn new_with_symbol(addr: usize, sym: String, source: Source) -> ACommand {
        ACommand {
            symbol_name: Some(sym),
            addr:        addr,
            value:       -1,
            source:      source,
        }
    }

    pub fn assign(self, symbols: &Symbols) -> ACommand {
        self.symbol_name
            .as_ref()
            .and_then(|name| {
                symbols.get(name.clone()).as_ref().map(|sym| {
                    let mut a = self.clone();
                    a.value = sym.addr as i64;
                    a
                })
            })
            .unwrap_or(self)
    }
}

pub fn parse(addr: usize, source: Source) -> Result<ACommand> {
    let name = source.code.get(1..).unwrap();

    if let Ok(num) = name.parse::<i64>() {
        Ok(ACommand::new_with_value(addr, num, source))
    } else {
        if !is_valid_symbol(name) {
            return Err(anyhow!("{:?} : invalid symbol name: {}", source, name));
        }
        let sym = String::from(name);
        Ok(ACommand::new_with_symbol(addr, sym, source))
    }
}

fn is_valid_symbol(name: &str) -> bool {
    let chars: Vec<char> = name.chars().collect();

    if let Some((head, rest)) = chars.split_first() {
        (is_valid_symbol_char(*head) && !head.is_ascii_digit())
            && rest.iter().all(|&c| is_valid_symbol_char(c))
    } else {
        false
    }
}

fn is_valid_symbol_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '$' || c == ':'
}
