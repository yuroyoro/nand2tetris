use crate::token::Token;

pub static mut TRACE_DEPTH: usize = 0;

pub struct Tracer<'a> {
    msg: &'a str,
    pub spacer: String,
}

impl Tracer<'_> {
    pub fn new<'a>(msg: &'a str) -> Tracer<'a> {
        let spacer = unsafe {
            let spacer = "  ".repeat(crate::parser::macros::TRACE_DEPTH);
            crate::parser::macros::TRACE_DEPTH += 1;
            spacer
        };
        Tracer { msg, spacer }
    }

    pub fn start(&self, t: Option<&Token>) {
        debug!("{}start : {} : {:?}", self.spacer, self.msg, t);
    }

    pub fn end(&self, t: Option<&Token>) {
        debug!("{}end : {} : {:?}", self.spacer, self.msg, t);
    }
}
impl Drop for Tracer<'_> {
    fn drop(&mut self) {
        unsafe {
            crate::parser::macros::TRACE_DEPTH -= 1;
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($stream:ident, $msg: literal, $blk: block) => {
        let tracer = crate::parser::macros::Tracer::new($msg);
        tracer.start($stream.current());

        let res = $blk;

        debug!(
            "{}end : {} : {:?} -> {:?}",
            tracer.spacer,
            $msg,
            $stream.current(),
            res
        );

        return res;
    };
}
