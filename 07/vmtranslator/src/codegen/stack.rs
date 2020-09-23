use crate::codegen::segment::*;
use crate::parser::Segment;

pub fn gen_cmd_push(vm_name: &str, seg: Segment, index: i64) -> String {
    vec![
        format!("// push {:?} {}", seg, index),
        gen_segment_read(vm_name, seg, index),
        gen_stack_push(),
    ]
    .join("\n")
}

// push D-register value to stack top
pub fn gen_stack_push() -> String {
    let asm = r#"@SP    // push stack
A=M
M=D
@SP
M=M+1
"#;

    String::from(asm)
}

// pop stack top value to  D-register
pub fn gen_stack_pop() -> String {
    let asm = r#"@SP    // pop stack
AM=M-1
D=M
"#;

    String::from(asm)
}

pub fn gen_cmd_pop(vm_name: &str, seg: Segment, index: i64) -> String {
    vec![
        format!("// pop {:?} {}", seg, index),
        gen_stack_pop(),
        gen_segment_write(vm_name, seg, index),
    ]
    .join("\n")
}
