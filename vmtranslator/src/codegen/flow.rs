use crate::codegen::stack::gen_stack_pop;
use crate::parser::Source;

use anyhow::Result;

pub fn gen_label(label: &str, _source: Source) -> Result<String> {
    let asm = format!(
        r#"
// label {}
({})
"#,
        label, label
    )
    .to_string();

    Ok(asm)
}

pub fn gen_goto(label: &str, _source: Source) -> Result<String> {
    let asm = format!(
        r#"
// goto {}
@{} // set destination label
0;JEQ
"#,
        label, label
    )
    .to_string();

    Ok(asm)
}

pub fn gen_if_goto(label: &str, _source: Source) -> Result<String> {
    let pop = gen_stack_pop()?;
    let asm = format!(
        r#"
// if-goto {}
{}
@{} // set destination label
D;JNE // jump if D(stack top value) != 0
"#,
        label, pop, label
    );

    Ok(asm)
}
