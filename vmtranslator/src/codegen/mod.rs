pub mod arithmetic;
pub mod flow;
pub mod function;
pub mod segment;
pub mod stack;

use crate::parser::{Command, ParseResult};

use std::collections::HashMap;

use anyhow::{Error, Result};

pub fn generate(results: Vec<ParseResult>) -> (String, Vec<Error>) {
    // search Sys.init
    let has_sys_init = results.iter().any(|res| {
        res.commands.iter().any(|cmd| match cmd {
            Command::Function(ref name, ..) => name == "Sys.init",
            _ => false,
        })
    });

    let mut table: LabelTable = HashMap::new();

    // generate prelude to call Sys.init
    let prelude = if has_sys_init {
        match gen_prelude(&mut table) {
            Ok(prelude) => prelude,
            Err(err) => return ("".to_string(), vec![err]),
        }
    } else {
        "".to_string()
    };

    let (codes, errors): (Vec<_>, Vec<_>) = results
        .into_iter()
        .flat_map(|res| gen(&res.vm_name, res.commands, &mut table))
        .partition(Result::is_ok);

    let codes: Vec<String> = codes.into_iter().map(|res| res.unwrap()).collect();
    let errors: Vec<Error> = errors.into_iter().map(|res| res.unwrap_err()).collect();

    let asm = format!("{}\n\n{}", prelude, codes.join("\n\n"));
    (asm, errors)
}

pub fn gen_prelude(table: &mut LabelTable) -> Result<String> {
    let call_sys_init = function::gen_call("prelude", "Sys.init", 0, table, None)?;

    // initialize stack pointer
    Ok(format!(
        r#"// prelude
@256
D=A
@SP
M=D

// entry point: call Sys.init
{}
"#,
        call_sys_init
    )
    .to_string())
}

pub type LabelTable = HashMap<String, i64>;

fn gen(vm_name: &str, commands: Vec<Command>, table: &mut LabelTable) -> Vec<Result<String>> {
    commands.into_iter().map(|cmd| gen_cmd(vm_name, cmd, table)).collect::<Vec<_>>()
}

fn gen_cmd(vm_name: &str, cmd: Command, table: &mut LabelTable) -> Result<String> {
    match cmd {
        // arithmetic commands
        Command::Add(source) => arithmetic::gen_add(source),
        Command::Sub(source) => arithmetic::gen_sub(source),
        Command::Neg(source) => arithmetic::gen_neg(source),
        Command::Eq(source) => arithmetic::gen_eq(table, source),
        Command::Gt(source) => arithmetic::gen_gt(table, source),
        Command::Lt(source) => arithmetic::gen_lt(table, source),
        Command::And(source) => arithmetic::gen_and(source),
        Command::Or(source) => arithmetic::gen_or(source),
        Command::Not(source) => arithmetic::gen_not(source),
        // memory access commands
        Command::Push(segment, index, source) => stack::gen_push(vm_name, segment, index, source),
        Command::Pop(segment, index, source) => stack::gen_pop(vm_name, segment, index, source),
        // program flow commands
        Command::Label(label, source) => flow::gen_label(&label, source),
        Command::Goto(label, source) => flow::gen_goto(&label, source),
        Command::IfGoto(label, source) => flow::gen_if_goto(&label, source),
        // function commands
        Command::Function(name, nlocals, source) => function::gen_function_def(vm_name, &name, nlocals, source),

        Command::Call(name, arity, source) => function::gen_call(vm_name, &name, arity, table, Some(source)),
        Command::Return(source) => function::gen_return(vm_name, source),
        // _ => Err(anyhow!("codegen: unexpected commnad: {:?}", cmd)),
    }
}

pub fn gen_new_label(op: &str, table: &mut LabelTable) -> String {
    // increment counter (or insert new entry)
    let cnt = table.entry(String::from(op)).and_modify(|e| *e += 1).or_insert(0);

    format!("{}_{}", op, cnt)
}
