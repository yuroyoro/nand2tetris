use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use super::symbols::Symbol;
use super::{Command, Segment};

use anyhow::{Context, Result};

pub struct Writer {
    classname: String,
    path: PathBuf,
    file: File,
}

impl Writer {
    pub fn new(classname: &str, path: PathBuf) -> Result<Writer> {
        let file = File::create(path.clone())?;

        Ok(Writer {
            classname: classname.to_string(),
            path: path,
            file: file,
        })
    }

    // push given segment value to stack
    pub fn push(&mut self, seg: Segment, index: usize) -> Result<()> {
        writeln!(self.file, "push {} {}", seg.display(), index)
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn push_constant(&mut self, n: u16) -> Result<()> {
        self.push(Segment::Const, n as usize)
    }

    pub fn push_from_args(&mut self, n: u16) -> Result<()> {
        self.push(Segment::Arg, n as usize)
    }

    pub fn push_from(&mut self, sym: &Symbol) -> Result<()> {
        if crate::CONFIG.debug {
            let comment = format!(
                "push_from name: {}, kind: {:?}, type: {:?}, index: {}",
                &sym.name, &sym.kind, &sym.typ, sym.index
            );
            return writeln!(
                self.file,
                "push {} {}\t\t\t\t// {}",
                sym.kind.segment().display(),
                sym.index,
                comment
            )
            .with_context(|| format!("failed to write vm file : {}", self.path.display()));
        }
        self.push(sym.kind.segment(), sym.index)
    }

    // pop from stack to given segment
    pub fn pop(&mut self, seg: Segment, index: usize) -> Result<()> {
        writeln!(self.file, "pop {} {}", seg.display(), index)
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn pop_to(&mut self, sym: &Symbol) -> Result<()> {
        if crate::CONFIG.debug {
            let comment = format!(
                "pop_to name: {}, kind: {:?}, type: {:?}, index: {}",
                &sym.name, &sym.kind, &sym.typ, sym.index
            );
            return writeln!(
                self.file,
                "pop {} {}\t\t\t\t// {}",
                sym.kind.segment().display(),
                sym.index,
                comment
            )
            .with_context(|| format!("failed to write vm file : {}", self.path.display()));
        }
        self.pop(sym.kind.segment(), sym.index)
    }

    pub fn set_this(&mut self) -> Result<()> {
        self.pop(Segment::Pointer, 0)
    }

    pub fn get_this(&mut self) -> Result<()> {
        self.push(Segment::Pointer, 0)
    }

    pub fn set_that(&mut self) -> Result<()> {
        self.pop(Segment::Pointer, 1)
    }

    pub fn get_that(&mut self) -> Result<()> {
        self.push(Segment::Pointer, 1)
    }

    pub fn pop_to_this(&mut self, index: usize) -> Result<()> {
        self.pop(Segment::This, index)
    }

    pub fn push_from_this(&mut self, index: usize) -> Result<()> {
        self.push(Segment::This, index)
    }

    pub fn pop_to_that(&mut self, index: usize) -> Result<()> {
        self.pop(Segment::That, index)
    }

    pub fn push_from_that(&mut self, index: usize) -> Result<()> {
        self.push(Segment::That, index)
    }

    pub fn arithmetic(&mut self, cmd: Command) -> Result<()> {
        writeln!(self.file, "{}", cmd.display())
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn label(&mut self, label: &str) -> Result<()> {
        writeln!(self.file, "label {}", label)
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn goto(&mut self, label: &str) -> Result<()> {
        writeln!(self.file, "goto {}", label)
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn if_goto(&mut self, label: &str) -> Result<()> {
        writeln!(self.file, "if-goto {}", label)
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn call(&mut self, name: &str, nargs: usize) -> Result<()> {
        writeln!(self.file, "call {} {}", name, nargs)
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn function(&mut self, name: &str, nlocals: usize) -> Result<()> {
        writeln!(
            self.file,
            "function {}.{} {}",
            self.classname, name, nlocals
        )
        .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn return_cmd(&mut self) -> Result<()> {
        writeln!(self.file, "return")
            .with_context(|| format!("failed to write vm file : {}", self.path.display()))
    }

    pub fn flush(&mut self) -> Result<()> {
        self.file
            .flush()
            .with_context(|| format!("failed to flush vm file : {}", self.path.display()))
    }
}
