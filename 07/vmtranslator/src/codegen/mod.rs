pub mod arithmetic;
pub mod segment;
pub mod stack;

use arithmetic::*;
use stack::*;

use crate::parser::Command;

use std::collections::HashMap;

pub fn generate(commands: Vec<Command>) -> String {
    vec![prelude(), gen(commands)].join("\n")
}

fn prelude() -> String {
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

fn gen(commands: Vec<Command>) -> String {
    let mut label_table: LabelTable = HashMap::new();

    commands
        .into_iter()
        .flat_map(|cmd| gen_cmd(cmd, &mut label_table))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

fn gen_cmd(cmd: Command, table: &mut LabelTable) -> Option<String> {
    match cmd {
        Command::Add => Some(gen_cmd_add()),
        Command::Sub => Some(gen_cmd_sub()),
        Command::Neg => Some(gen_cmd_neg()),
        Command::Eq => Some(gen_cmd_eq(table)),
        Command::Gt => Some(gen_cmd_gt(table)),
        Command::Lt => Some(gen_cmd_lt(table)),
        Command::And => Some(gen_cmd_and()),
        Command::Or => Some(gen_cmd_or()),
        Command::Not => Some(gen_cmd_not()),
        Command::Push(segment, index) => Some(gen_cmd_push(segment, index)),
        Command::Pop(segment, index) => Some(gen_cmd_pop(segment, index)),
        _ => None,
    }
}
