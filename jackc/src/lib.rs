#[macro_use]
extern crate lazy_static;
extern crate getopts;

#[macro_use]
pub mod macros;

pub mod codegen;
pub mod parser;
pub mod source;
pub mod token;

use parser::ASTs;
use source::SourceIter;
use token::Tokens;

use std::env;
use std::process;
use std::rc::Rc;

use getopts::Options;

use anyhow::Result;

#[derive(PartialEq)]
pub enum Mode {
    Tokenize,
    Parse,
    Compile,
}

pub struct Config {
    pub mode: Mode,
    pub debug: bool,
    pub target: String,
}

lazy_static! {
    pub static ref CONFIG: Config = parse_args();
}

/**
 * 1. Read file or directory
 * 2. tokenize each .jack files to Token(s)
 * 3. parse each .jack files to AST
 * 4. generate vm commands from AST
 */
pub fn process() {
    // source iterator
    let sources = source::read_sources(&CONFIG.target).unwrap_or_else(|err| {
        println!("cannot read file: {}", err);
        process::exit(1);
    });

    // Tokenize
    let tokens_list = process_tokenize(sources);

    // Parse
    let asts_list = process_parse(tokens_list);

    process_codegen(asts_list);

    process::exit(0);
}

fn process_tokenize(sources: SourceIter) -> Vec<Tokens> {
    let (results, errors): (Vec<_>, Vec<_>) = sources
        .map(|rs| rs.and_then(|rcsrc| token::tokenize(Rc::new(rcsrc))))
        .partition(Result::is_ok);

    handle_errors("tokenize", errors);

    if CONFIG.mode == Mode::Tokenize {
        results
            .into_iter()
            .map(|tokens| token::write_tokens(tokens.unwrap()))
            .collect::<Result<()>>()
            .unwrap_or_else(|err| {
                println!("failed to write xml {}", err);
                process::exit(1);
            });
        process::exit(0);
    }

    results.into_iter().map(|tk| tk.unwrap()).collect()
}

fn process_parse(tokens_list: Vec<Tokens>) -> Vec<ASTs> {
    let (results, errors): (Vec<_>, Vec<_>) = tokens_list
        .into_iter()
        .map(|tokens| parser::parse(tokens))
        .partition(Result::is_ok);

    handle_errors("parse", errors);

    if CONFIG.mode == Mode::Parse {
        results
            .into_iter()
            .map(|asts| parser::write_asts(asts.unwrap()))
            .collect::<Result<()>>()
            .unwrap_or_else(|err| {
                println!("failed to write xml {}", err);
                process::exit(1);
            });

        process::exit(0);
    }

    results.into_iter().map(|asts| asts.unwrap()).collect()
}

fn process_codegen(asts_list: Vec<ASTs>) {
    let (_, errors): (Vec<_>, Vec<_>) = codegen::gen(asts_list);

    handle_errors("codegen", errors);
}

fn handle_errors<T: std::fmt::Debug>(msg: &str, errors: Vec<Result<T>>) -> () {
    if !errors.is_empty() {
        println!("{} error: ", msg);
        errors
            .into_iter()
            .for_each(|err| println!("  {:?}", err.unwrap_err()));
        process::exit(1);
    }
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag(
        "",
        "tokenize",
        "tokenize sources and write xml to *T.result.xml",
    );
    opts.optflag("", "parse", "parse to ast and write xml to *.result.xm");
    opts.optflag("v", "verbose", "print debug logs");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            process::exit(1);
        }
    };

    let mut mode = Mode::Compile;
    if matches.opt_present("tokenize") {
        mode = Mode::Tokenize;
    }
    if matches.opt_present("parse") {
        mode = Mode::Parse;
    }

    let debug = matches.opt_present("v");

    let target = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("not enough arguments");
        process::exit(1);
    };

    Config {
        mode,
        debug,
        target,
    }
}

pub fn to_lowercase_first_char(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    format!("{}{}", head.to_lowercase(), tail)
}

pub fn to_uppercase_first_char(s: &str) -> String {
    let (head, tail) = s.split_at(1);
    format!("{}{}", head.to_uppercase(), tail)
}
