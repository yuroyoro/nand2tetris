use crate::codegen::stack::gen_stack_pop;
use crate::codegen::{gen_new_label, LabelTable};
use crate::parser::Source;

use anyhow::Result;

pub fn gen_add(_source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let asm = r#"@SP    // replace stack top by D+M
A=M-1
M=D+M"#;

    Ok(format!("// add\n{}\n{}", pop, asm))
}

pub fn gen_sub(_source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let asm = r#"@SP    // replace stack top by D-M
A=M-1
M=M-D"#;

    Ok(format!("// sub\n{}\n{}", pop, asm))
}

pub fn gen_neg(_source: Source) -> Result<String> {
    let asm = r#"@SP    // replace stack top by !M
A=M-1
M=-M"#;

    Ok(format!("// neg\n{}", asm))
}

pub fn gen_eq(table: &mut LabelTable, _source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let label = gen_new_label("END_EQ", table);
    Ok(format!(
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
    ))
}

pub fn gen_gt(table: &mut LabelTable, _source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let label = gen_new_label("END_GT", table);
    Ok(format!(
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
    ))
}

pub fn gen_lt(table: &mut LabelTable, _source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let label = gen_new_label("END_LT", table);
    Ok(format!(
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
    ))
}

pub fn gen_and(_source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let asm = r#"@SP    // replace stack top by D&M
A=M-1
M=D&M"#;

    Ok(format!("// and\n{}\n{}", pop, asm))
}

pub fn gen_or(_source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let asm = r#"@SP    // replace stack top by D|M
A=M-1
M=D|M"#;

    Ok(format!("// or\n{}\n{}", pop, asm))
}

pub fn gen_not(_source: Source) -> Result<String> {
    let asm = r#"@SP    // replace stack top by !M
A=M-1
M=!M"#;

    Ok(format!("// not\n{}", asm))
}
