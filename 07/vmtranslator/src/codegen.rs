use super::parser::{Command, Segment};

pub fn generate(commands: Vec<Command>) -> String {
    vec![prelude(), gen(commands)].join("\n")
}

fn prelude() -> String {
    // initialize stack pointer
    let asm = r#"// prelude
@256
D=A
@SP
M=D
"#;
    String::from(asm)
}

fn gen(commands: Vec<Command>) -> String {
    commands
        .into_iter()
        .flat_map(|cmd| gen_cmd(cmd))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}
fn gen_cmd(cmd: Command) -> Option<String> {
    match cmd {
        Command::Add => Some(gen_cmd_add()),
        Command::Push(segment, index) => Some(gen_cmd_push(segment, index)),
        // Command::Pop(segment, index) => gen_cmd_pop(segment, index),
        _ => None,
    }
}

fn gen_cmd_add() -> String {
    let pop = gen_stack_pop();
    let asm = r#"@SP    // replace stack top by D+M
A=M-1
M=D+M"#;

    format!("// add\n{}\n{}", pop, asm)
}

fn gen_cmd_push(seg: Segment, index: i64) -> String {
    vec![
        format!("// push {:?} {}", seg, index),
        gen_segment(seg, index),
        gen_stack_push(),
    ]
    .join("\n")
}

// push D-register value to stack top
fn gen_stack_push() -> String {
    let asm = r#"@SP    // push stack
A=M
M=D
@SP
M=M+1
"#;

    String::from(asm)
}

// pop stack top value to  D-register
fn gen_stack_pop() -> String {
    let asm = r#"@SP    // pop stack
A=M-1
D=M
@SP
M=M-1
"#;

    String::from(asm)
}

// fn gen_cmd_pop(seg: Segment, index: i64) -> String {
//     gen_segment(seg, index)
//         + r#"
// @SP
// A=M-1
// D=M
// M=D"#;
// }

fn gen_segment(seg: Segment, index: i64) -> String {
    match seg {
        Segment::Constant => gen_segment_constant(index),
        _ => String::from(""), // TODO
    }
}

fn gen_segment_constant(index: i64) -> String {
    if index >= 0 {
        format!("@{}\nD=A    // constant", index)
    } else {
        format!("@{}\nD=-A   // constant", -index)
    }
}
