extern crate hackasm;

use hackasm::codegen;
use hackasm::parser;
use hackasm::symbols::Symbols;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::process;
/**
 * 1. parse
 * 2. resolve symbols
 * 3. codegen
 */

fn main() {
    // get filename from args
    let filename = get_filename();

    // check filename ext
    valiate_filename(&filename);

    // read file
    let contents = read_source(&filename);

    let mut symbols = Symbols::new();

    // parse
    let (nodes, errors) = parser::parse(&contents, &mut symbols);

    if !errors.is_empty() {
        println!("parse error: ");
        errors.into_iter().for_each(|err| println!("  {:?}", err));
        process::exit(1);
    }

    // resolve symbols
    let nodes = symbols.resolve(nodes);

    // println!("Nodes: ");
    // nodes.iter().for_each(|node| println!("  {:?}", node));

    // generate code
    let mcodes = codegen::generate(nodes);

    // write to file
    write_mcodes(&filename, &mcodes);
}

fn get_filename() -> String {
    // get filename from args
    let args: Vec<String> = env::args().collect();
    let name = args.get(1).unwrap_or_else(|| {
        println!("not enough arguments");
        process::exit(1);
    });
    String::from(name)
}

fn valiate_filename(filename: &str) {
    if !filename.ends_with(".asm") {
        println!("invalid filename, exptected to '*.asm': {:?}", filename);
        process::exit(1);
    }
}

fn read_source(filename: &str) -> String {
    // read file
    let mut contents = String::new();
    File::open(filename)
        .unwrap_or_else(|err| {
            println!("cannot read file: {}", err);
            process::exit(1);
        })
        .read_to_string(&mut contents)
        .unwrap_or_else(|err| {
            println!("cannot read file: {}", err);
            process::exit(1);
        });

    contents
}

fn write_mcodes(filename: &str, mcodes: &str) {
    let names = filename.rsplitn(2, ".").collect::<Vec<&str>>();
    if let [_ext, name] = &*names {
        let filename = format!("{}.hack", name);
        File::create(&filename)
            .unwrap_or_else(|err| {
                println!("cannot open file: {}", err);
                process::exit(1);
            })
            .write_all(mcodes.as_bytes())
            .unwrap_or_else(|err| {
                println!("cannot write file: {}", err);
                process::exit(1);
            });
        println!("write mcodes to {}", &filename);
    }
}
