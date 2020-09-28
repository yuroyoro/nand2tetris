use crate::codegen::segment::*;
use crate::parser::Segment;
use crate::parser::Source;

use anyhow::Result;

pub fn gen_push(vm_name: &str, seg: Segment, index: i64, _source: Source) -> Result<String> {
    let asm = vec![format!("// push {:?} {}", seg, index), gen_segment_read(vm_name, seg, index), gen_stack_push()?].join("\n");

    Ok(asm)
}

// push D-register value to stack top
pub fn gen_stack_push() -> Result<String> {
    let asm = r#"@SP    // push stack
A=M
M=D
@SP
M=M+1
"#;

    Ok(asm.to_string())
}

// pop stack top value to  D-register
pub fn gen_stack_pop() -> Result<String> {
    let asm = r#"@SP    // pop stack to D-register
AM=M-1
D=M
"#;

    Ok(asm.to_string())
}

pub fn gen_pop(vm_name: &str, seg: Segment, index: i64, _source: Source) -> Result<String> {
    let asm = vec![format!("// pop {:?} {}", seg, index), gen_stack_pop()?, gen_segment_write(vm_name, seg, index)].join("\n");

    Ok(asm)
}
