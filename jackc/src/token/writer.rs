use crate::token::*;

use std::fs::File;
use std::io::prelude::*;

use anyhow::{Context, Result};

pub fn write_tokens(tokens: Tokens) -> Result<()> {
    let path = tokens.source.xml_filename()?;

    let mut f = File::create(path)?;

    writeln!(f, "<tokens>")?;
    tokens
        .tokens
        .iter()
        .map(|token| writeln!(f, "{}", to_xml(token)).context("failed to write xml"))
        .collect::<Result<_>>()?;

    writeln!(f, "</tokens>")?;
    f.flush().context("falied to flush xml file")
}

fn to_xml(token: &Token) -> String {
    match token {
        Token::Symbol(s, _) => match &*s.as_str() {
            "<" => format!("<symbol> &lt; </symbol>"),
            ">" => format!("<symbol> &gt; </symbol>"),
            "&" => format!("<symbol> &amp; </symbol>"),
            _ => format!("<symbol> {} </symbol>", s),
        },
        Token::Keyword(kwd, _) => format!(
            "<keyword> {} </keyword>",
            to_lowercase_first_char(format!("{:?}", kwd).as_str())
        ),
        Token::Integer(n, _) => format!("<integerConstant> {} </integerConstant>", n),
        Token::Str(s, _) => format!("<stringConstant> {} </stringConstant>", s),
        Token::Identifier(s, _) => format!("<identifier> {} </identifier>", s),
        _ => "<unknown/>".to_string(),
    }
}
