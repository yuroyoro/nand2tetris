use crate::parser::Segment;

pub fn gen_segment_read(seg: Segment, index: i64) -> String {
    match seg {
        Segment::Constant => gen_segment_read_constant(index),
        Segment::Local => gen_segment_read_local(index),
        Segment::Argument => gen_segment_read_argument(index),
        Segment::This => gen_segment_read_this(index),
        Segment::That => gen_segment_read_that(index),
        Segment::Temp => gen_segment_read_temp(index),
        _ => String::from(""), // TODO
    }
}

fn gen_segment_read_constant(index: i64) -> String {
    if index >= 0 {
        format!("@{}\nD=A    // constant", index)
    } else {
        format!("@{}\nD=-A   // constant", -index)
    }
}

fn gen_segment_read_local(index: i64) -> String {
    gen_segment_index_access("local", "LCL", index)
}

fn gen_segment_read_argument(index: i64) -> String {
    gen_segment_index_access("argument", "ARG", index)
}

fn gen_segment_read_this(index: i64) -> String {
    gen_segment_index_access("this", "THIS", index)
}

fn gen_segment_read_that(index: i64) -> String {
    gen_segment_index_access("that", "THAT", index)
}

fn gen_segment_read_temp(index: i64) -> String {
    let reg = 5 + index;
    format!(
        r#"@R{} // set (temp {}) to D-register
D=M
"#,
        reg, index
    )
    .to_string()
}

fn gen_segment_index_access(name: &str, segment: &str, index: i64) -> String {
    format!(
        r#"@{} // set ({} {}) to D-register
D=A
@{}
A=M+A
D=M
"#,
        index, name, index, segment
    )
    .to_string()
}

pub fn gen_segment_write(seg: Segment, index: i64) -> String {
    match seg {
        Segment::Local => gen_segment_write_local(index),
        Segment::Argument => gen_segment_write_argument(index),
        Segment::This => gen_segment_write_this(index),
        Segment::That => gen_segment_write_that(index),
        Segment::Temp => gen_segment_write_temp(index),
        _ => String::from(""), // TODO
    }
}

fn gen_segment_write_local(index: i64) -> String {
    gen_segment_index_write("local", "LCL", index)
}

fn gen_segment_write_argument(index: i64) -> String {
    gen_segment_index_write("argument", "ARG", index)
}

fn gen_segment_write_this(index: i64) -> String {
    gen_segment_index_write("this", "THIS", index)
}

fn gen_segment_write_that(index: i64) -> String {
    gen_segment_index_write("that", "THAT", index)
}

fn gen_segment_write_temp(index: i64) -> String {
    let reg = 5 + index;
    format!(
        r#"@R{} // write d-register value to (temp {})
M=D
"#,
        reg, index
    )
    .to_string()
}

fn gen_segment_index_write(name: &str, segment: &str, index: i64) -> String {
    let incr = "A=A+1\n".repeat(index as usize);
    format!(
        r#"@{} // write d-register value to ({} {})
A=M
{}
M=D
"#,
        segment, name, index, incr
    )
    .to_string()
}
