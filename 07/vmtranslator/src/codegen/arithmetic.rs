use crate::codegen::stack::*;
use crate::codegen::LabelTable;

pub fn gen_cmd_add() -> String {
    let pop = gen_stack_pop();
    let asm = r#"@SP    // replace stack top by D+M
A=M-1
M=D+M"#;

    format!("// add\n{}\n{}", pop, asm)
}

pub fn gen_cmd_sub() -> String {
    let pop = gen_stack_pop();
    let asm = r#"@SP    // replace stack top by D-M
A=M-1
M=M-D"#;

    format!("// sub\n{}\n{}", pop, asm)
}

pub fn gen_cmd_neg() -> String {
    let asm = r#"@SP    // replace stack top by !M
A=M-1
M=-M"#;

    format!("// neg\n{}", asm)
}

fn gen_new_label(op: &str, table: &mut LabelTable) -> String {
    // increment counter (or insert new entry)
    let cnt = table
        .entry(String::from(op))
        .and_modify(|e| *e += 1)
        .or_insert(0);

    format!("{}_{}", op, cnt)
}

pub fn gen_cmd_eq(table: &mut LabelTable) -> String {
    let pop = gen_stack_pop();
    let label = gen_new_label("END_EQ", table);
    format!(
        r#"
// eq
{}
@SP
A=M-1
D=M-D
M=-1 // set true to stack top
@{}
D;JEQ
@SP
A=M-1
M=0 // set false to stack top
({})
"#,
        pop, label, label
    )
}

pub fn gen_cmd_gt(table: &mut LabelTable) -> String {
    let pop = gen_stack_pop();
    let label = gen_new_label("END_GT", table);
    format!(
        r#"
// gt
{}
@SP
A=M-1
D=M-D
M=-1 // set true to stack top
@{}
D;JGT
@SP
A=M-1
M=0 // set false to stack top
({})
"#,
        pop, label, label
    )
}

pub fn gen_cmd_lt(table: &mut LabelTable) -> String {
    let pop = gen_stack_pop();
    let label = gen_new_label("END_LT", table);
    format!(
        r#"
// lt
{}
@SP
A=M-1
D=M-D
M=-1 // set true to stack top
@{}
D;JLT
@SP
A=M-1
M=0 // set false to stack top
({})
"#,
        pop, label, label
    )
}

pub fn gen_cmd_and() -> String {
    let pop = gen_stack_pop();
    let asm = r#"@SP    // replace stack top by D&M
A=M-1
M=D&M"#;

    format!("// and\n{}\n{}", pop, asm)
}

pub fn gen_cmd_or() -> String {
    let pop = gen_stack_pop();
    let asm = r#"@SP    // replace stack top by D|M
A=M-1
M=D|M"#;

    format!("// or\n{}\n{}", pop, asm)
}

pub fn gen_cmd_not() -> String {
    let asm = r#"@SP    // replace stack top by !M
A=M-1
M=!M"#;

    format!("// not\n{}", asm)
}
