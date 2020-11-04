pub mod parser;
pub mod source;
pub mod tokenizer;

use parser::{parse, write_asts};
use source::read_sources;
use tokenizer::{tokenize, write_tokens};

use std::env;
use std::process;
use std::rc::Rc;

use anyhow::Result;

#[derive(PartialEq)]
enum Mode {
    Tokenize,
    Parse,
}

/**
 * 1. Read file or directory
 * 2. tokenize each .jack files to Token(s)
 * 3. parse each .jack files to AST
 * 4. generate xml from AST
 */
pub fn process() {
    // get filename or directory from args
    let (mode, arg) = parse_args();

    // source iterator
    let sources = read_sources(&arg)
        .unwrap_or_else(|err| {
            println!("cannot read file: {}", err);
            process::exit(1);
        })
        .map(|ressrc| ressrc.map(Rc::new)); // wrap sources by Rc

    // Tokenize
    let (results, errors): (Vec<_>, Vec<_>) = sources
        .map(|rs| rs.and_then(|rcsrc| tokenize(rcsrc)))
        .partition(Result::is_ok);

    handle_errors("tokenize", errors);

    if mode == Mode::Tokenize {
        results
            .into_iter()
            .map(|tokens| write_tokens(tokens.unwrap()))
            .collect::<Result<()>>()
            .unwrap_or_else(|err| {
                println!("failed to write xml {}", err);
                process::exit(1);
            });
        process::exit(0);
    }

    // Parse
    let (results, errors): (Vec<_>, Vec<_>) = results
        .into_iter()
        .map(|tokens| parse(tokens.unwrap()))
        .partition(Result::is_ok);

    handle_errors("parse", errors);

    results
        .into_iter()
        .for_each(|asts| write_asts(asts.unwrap()));

    process::exit(0);
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

fn parse_args() -> (Mode, String) {
    // get first arg
    let args: Vec<String> = env::args().collect();

    let error_handler = || {
        println!("not enough arguments");
        process::exit(1);
    };

    let arg = args.get(1).unwrap_or_else(error_handler);
    let get_2nd_arg = || String::from(args.get(2).unwrap_or_else(error_handler));

    match arg.as_str() {
        "tokenize" => (Mode::Tokenize, get_2nd_arg()),
        "parse" => (Mode::Parse, get_2nd_arg()),
        _ => (Mode::Parse, String::from(arg)),
    }
}
