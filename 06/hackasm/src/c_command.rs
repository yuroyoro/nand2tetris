use super::parser::*;

use anyhow::{anyhow, Result};
use maplit::hashmap;

use std::collections::HashMap;

#[derive(Debug)]
pub struct CCommand {
    pub dest: Dest,
    pub comp: Comp,
    pub jump: Jump,
    pub addr: usize,
    source:   Source,
}

// C-Command dest operand
#[derive(PartialEq, Clone, Copy, Debug, enum_utils::FromStr)]
pub enum Dest {
    Null = 0b000,
    M    = 0b001,
    D    = 0b010,
    MD   = 0b011,
    A    = 0b100,
    AM   = 0b101,
    AD   = 0b110,
    AMD  = 0b111,
}

// C-Command jump opreand
#[derive(PartialEq, Clone, Copy, Debug, enum_utils::FromStr)]
pub enum Jump {
    Null = 0b000,
    JGT  = 0b001,
    JEQ  = 0b010,
    JGE  = 0b011,
    JLT  = 0b100,
    JNE  = 0b101,
    JLE  = 0b110,
    JMP  = 0b111,
}

// C-Command comp operand
#[derive(Debug, Copy, Clone)]
pub struct Comp {
    exp:       &'static str, // expression
    pub mcode: i8,           // machien code (7bit)
}

lazy_static! {
    pub static ref COMP_MAP: HashMap<&'static str, Comp> = hashmap!(
        "0"   => Comp{ exp: "0",   mcode: 0b0101010 },
        "1"   => Comp{ exp: "1",   mcode: 0b0111111 },
        "-1"  => Comp{ exp: "-1",  mcode: 0b0111010 },
        "D"   => Comp{ exp: "D",   mcode: 0b0001100 },
        "A"   => Comp{ exp: "A",   mcode: 0b0110000 },
        "!D"  => Comp{ exp: "!D",  mcode: 0b0001101 },
        "!A"  => Comp{ exp: "!A",  mcode: 0b0110001 },
        "-D"  => Comp{ exp: "-D",  mcode: 0b0001111 },
        "-A"  => Comp{ exp: "-A",  mcode: 0b0110011 },
        "D+1" => Comp{ exp: "D+1", mcode: 0b0011111 },
        "A+1" => Comp{ exp: "A+1", mcode: 0b0110111 },
        "D-1" => Comp{ exp: "D-1", mcode: 0b0001110 },
        "A-1" => Comp{ exp: "A-1", mcode: 0b0110010 },
        "D+A" => Comp{ exp: "D+A", mcode: 0b0000010 },
        "D-A" => Comp{ exp: "D-A", mcode: 0b0010011 },
        "A-D" => Comp{ exp: "A-D", mcode: 0b0000111 },
        "D&A" => Comp{ exp: "D&A", mcode: 0b0000000 },
        "D|A" => Comp{ exp: "D|A", mcode: 0b0010101 },
        "M"   => Comp{ exp: "M",   mcode: 0b1110000 },
        "!M"  => Comp{ exp: "!M",  mcode: 0b1110001 },
        "-M"  => Comp{ exp: "-M",  mcode: 0b1110011 },
        "M+1" => Comp{ exp: "M+1", mcode: 0b1110111 },
        "M-1" => Comp{ exp: "M-1", mcode: 0b1110010 },
        "D+M" => Comp{ exp: "D+M", mcode: 0b1000010 },
        "D-M" => Comp{ exp: "D-M", mcode: 0b1010011 },
        "M-D" => Comp{ exp: "M-D", mcode: 0b1000111 },
        "D&M" => Comp{ exp: "D&M", mcode: 0b1000000 },
        "D|M" => Comp{ exp: "D|M", mcode: 0b1010101 },
    );
}

pub fn parse(addr: usize, source: Source) -> Result<CCommand> {
    let (dest, lhs) = split_code(&source.code, "=", true);
    let dest = dest.unwrap_or("Null");
    let dest = parse_dest(dest)?;

    let lhs = lhs.ok_or(anyhow!(
        "{:?} :lhs operand is missing: {}",
        source,
        source.code
    ))?;

    let (comp, jump) = parse_comp_and_jmp(&source, lhs)?;

    let cmd = CCommand {
        dest,
        comp,
        jump,
        addr,
        source,
    };
    Ok(cmd)
}

fn parse_dest(dest: &str) -> Result<Dest> {
    dest.parse::<Dest>()
        .map_err(|()| anyhow!("invalid dest operand: {}", dest))
}

fn parse_comp_and_jmp(source: &Source, code: &str) -> Result<(Comp, Jump)> {
    let (comp, jump) = split_code(&code, ";", false);
    let comp = comp.ok_or(anyhow!("{:?} : comp operand is missing: {}", source, code))?;
    let comp = COMP_MAP
        .get(comp)
        .ok_or(anyhow!("unnown comp operand : {}", comp))?;
    let jump = jump.unwrap_or("Null");
    let jump = parse_jump(source, jump)?;

    Ok((*comp, jump))
}

fn parse_jump(source: &Source, jump: &str) -> Result<Jump> {
    jump.parse::<Jump>()
        .map_err(|()| anyhow!("{:?} : invalid jmp operand: {}", source, jump))
}
