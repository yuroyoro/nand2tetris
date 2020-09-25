pub mod arithmetic;
pub mod flow;
pub mod segment;
pub mod stack;

use crate::parser::{Command, ParseResult};

use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};

pub fn generate(results: Vec<ParseResult>) -> (String, Vec<Error>) {
    let prelude = gen_prelude();

    let (codes, errors): (Vec<_>, Vec<_>) = results
        .into_iter()
        .flat_map(|res| gen(&res.vm_name, res.commands))
        .partition(Result::is_ok);

    let codes: Vec<String> = codes.into_iter().map(|res| res.unwrap()).collect();
    let errors: Vec<Error> = errors.into_iter().map(|res| res.unwrap_err()).collect();

    let asm = format!("{}\n\n{}", prelude, codes.join("\n\n"));
    (asm, errors)
}

pub fn gen_prelude() -> String {
    // initialize stack pointer, segment base address
    let asm = r#"// prelude
@256
D=A
@SP
M=D
"#;
    String::from(asm)
}

pub type LabelTable = HashMap<String, i64>;

fn gen(vm_name: &str, commands: Vec<Command>) -> Vec<Result<String>> {
    let mut label_table: LabelTable = HashMap::new();

    commands
        .into_iter()
        .map(|cmd| gen_cmd(vm_name, cmd, &mut label_table))
        .collect::<Vec<_>>()
}

fn gen_cmd(vm_name: &str, cmd: Command, table: &mut LabelTable) -> Result<String> {
    match cmd {
        Command::Add(source) => arithmetic::gen_add(source),
        Command::Sub(source) => arithmetic::gen_sub(source),
        Command::Neg(source) => arithmetic::gen_neg(source),
        Command::Eq(source) => arithmetic::gen_eq(table, source),
        Command::Gt(source) => arithmetic::gen_gt(table, source),
        Command::Lt(source) => arithmetic::gen_lt(table, source),
        Command::And(source) => arithmetic::gen_and(source),
        Command::Or(source) => arithmetic::gen_or(source),
        Command::Not(source) => arithmetic::gen_not(source),
        Command::Push(segment, index, source) => stack::gen_push(vm_name, segment, index, source),
        Command::Pop(segment, index, source) => stack::gen_pop(vm_name, segment, index, source),
        Command::Label(label, source) => flow::gen_label(&label, source),
        Command::Goto(label, source) => flow::gen_goto(&label, source),
        Command::IfGoto(label, source) => flow::gen_if_goto(&label, source),
        _ => Err(anyhow!("codegen: unexpected commnad: {:?}", cmd)),
    }
}
