use super::*;
use crate::parser::ast::*;

use anyhow::Result;

pub fn write_class(w: &mut Writer, types: &Types, cls: &Class) -> Result<()> {
    let mut symbols = Symbols::class(&types, cls);

    cls.subroutines
        .iter()
        .map(|sub| function::write_function(w, &mut symbols, cls, sub))
        .collect::<Result<()>>()
}
