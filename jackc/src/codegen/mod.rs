mod class;
mod expr;
mod function;
mod statement;
mod symbols;
mod typedef;
mod writer;

use crate::parser::ast::*;
use crate::to_lowercase_first_char;

pub use symbols::Symbols;
pub use typedef::Types;
pub use writer::Writer;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Segment {
    Const,
    Static,
    Arg,
    Local,
    This,
    That,
    Pointer,
    Temp,
}

impl Segment {
    pub fn display(&self) -> String {
        match self {
            Segment::Const => "constant".to_string(),
            Segment::Arg => "argument".to_string(),
            _ => to_lowercase_first_char(format!("{:?}", self).as_str()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl Command {
    pub fn display(&self) -> String {
        to_lowercase_first_char(format!("{:?}", self).as_str())
    }
}

pub fn gen(asts_list: Vec<ASTs>) -> (Vec<Result<()>>, Vec<Result<()>>) {
    let mut types = Types::new();
    types.define_classes(&asts_list);

    asts_list
        .into_iter()
        .map(|asts| gen_vm(asts, &types))
        .partition(Result::is_ok)
}

pub fn gen_vm(asts: ASTs, types: &Types) -> Result<()> {
    let path = asts.source.vm_filename()?;
    let mut w = Writer::new(&asts.class.name, path)?;

    class::write_class(&mut w, types, &asts.class)?;
    w.flush()
}
