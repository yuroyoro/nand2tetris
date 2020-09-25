use crate::parser::Source;
use anyhow::{anyhow, Result};

pub fn validate_symbol(symbol: &str, source: &Source) -> Result<String> {
    let chars: Vec<char> = symbol.chars().collect();

    let valid = if let Some((head, rest)) = chars.split_first() {
        (is_valid_symbol_char(*head) && !head.is_ascii_digit())
            && rest.iter().all(|&c| is_valid_symbol_char(c))
    } else {
        false
    };

    if valid {
        Ok(symbol.to_string())
    } else {
        Err(anyhow!("{:?} : invalid symbol name : {}", &source, symbol))
    }
}

pub fn is_valid_symbol_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '$' || c == ':'
}
