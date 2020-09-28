use std::iter;

use crate::codegen::stack::gen_stack_push;
use crate::codegen::{gen_new_label, LabelTable};
use crate::parser::Source;

use anyhow::Result;

pub fn gen_function_def(_vm_name: &str, name: &str, nlocals: i64, _source: Source) -> Result<String> {
    let push = gen_stack_push()?;
    let locals = iter::repeat(push).take(nlocals as usize).collect::<Vec<String>>().join("\n");

    Ok(format!(
        r#"
// function {} {}
({})

@0        // initialize local segments
D=A
{}
"#,
        name, nlocals, name, locals
    )
    .to_string())
}

pub fn gen_call(_vm_name: &str, name: &str, arity: i64, table: &mut LabelTable, _source: Option<Source>) -> Result<String> {
    let retaddr = gen_new_label("RET_ADDR_CALL", table);
    let push = gen_stack_push()?;

    Ok(format!(
        r#"
@{}     // push return address
D=A
{}

@LCL    // push local segment pointer
D=M
{}
@ARG    // push argument segment pointer
D=M
{}
@THIS   // push this segment pointer
D=M
{}
@THAT   // push that segment pointer
D=M
{}

        // set argument segment pointer (@ARG=SP-5-arity)
@{}     // arity + 5
D=A
@SP
D=M-D   // @SP - (5 + arity)
@ARG
M=D     // ARG = @SP - 5 - arity

        // set stack top address to local segment pointer
@SP
D=M
@LCL
M=D

        // jump to target function
@{}
0;JEQ

        // return address
({})

"#,
        retaddr,
        push,
        push,
        push,
        push,
        push,
        arity + 5,
        name,
        retaddr
    )
    .to_string())
}

pub fn gen_return(_vm_name: &str, _source: Source) -> Result<String> {
    let asm = r#"
// return
@LCL    // R13(FRAME) = LCL
D=M
@R13
M=D

@5      // R14(RET) = *(FRAME - 5)
A=D-A
D=M
@R14
M=D

@SP     // R15(RETVAL) = *(SP - 1)
A=M-1
D=M
@R15
M=D

@ARG    // restore SP (SP = *ARG + 1)
D=M+1
@SP
M=D

@R15    // set retval (*(SP - 1) = R15(RETVAL)
D=M
@SP
A=M-1
M=D

@R13    // restore THAT = *(FRAME - 1)
A=M-1
D=M
@THAT
M=D

@2      // restore THIS = *(FRAME - 2)
D=A
@R13
A=M-D
D=M
@THIS
M=D

@3      // restore ARG = *(FRAME - 3)
D=A
@R13
A=M-D
D=M
@ARG
M=D

@4      // restore LCL = *(FRAME - 4)
D=A
@R13
A=M-D
D=M
@LCL
M=D

@R14    // jump to return address
A=M
0;JEQ
"#;

    Ok(asm.to_string())
}
