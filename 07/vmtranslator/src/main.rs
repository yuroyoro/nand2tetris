extern crate vmtranslator;

use vmtranslator::codegen::*;
use vmtranslator::parser::*;
use vmtranslator::source::*;

use std::env;
use std::process;

use std::fs::File;
use std::io::Write;

/**
 * 1. Read file or directory
 * 2. parse each vm files to VMCommand(s)
 * 3. generate hack asm from VMCommand(s)
 *
 */
fn main() {
    // get filename or directory from args
    let arg = get_first_arg();
    let vm_name = get_vm_name(&arg);

    let sources = read_sources(&arg).unwrap_or_else(|err| {
        println!("cannot read file: {}", err);
        process::exit(1);
    });

    let results: Vec<ParseResult> = sources.map(|src| parse(&src.code, &src.vm_name)).collect();
    let errors: Vec<&anyhow::Error> = results.iter().map(|res| &res.errors).flatten().collect();

    if !errors.is_empty() {
        println!("parse error: ");
        errors.into_iter().for_each(|err| println!("  {:?}", err));
        process::exit(1);
    }

    // generate code
    let asm = results
        .into_iter()
        .map(|res| generate(&res.vm_name, res.commands))
        .collect::<Vec<String>>()
        .join("\n\n");

    // write to file
    write_asm(&vm_name, &asm);

    println!("Asm:");
    println!("{}", &asm);
}

fn get_first_arg() -> String {
    // get first arg
    let args: Vec<String> = env::args().collect();
    let name = args.get(1).unwrap_or_else(|| {
        println!("not enough arguments");
        process::exit(1);
    });
    String::from(name)
}

fn get_vm_name(filename: &str) -> String {
    if filename.ends_with(".vm") {
        filename.replace(".vm", "")
    } else {
        format!("{}.asm", filename.strip_suffix("/").unwrap_or(filename))
    }
}

fn write_asm(vm_name: &str, asm: &str) {
    let filename = format!("{}.asm", vm_name);

    File::create(&filename)
        .unwrap_or_else(|err| {
            println!("cannot open file: {}", err);
            process::exit(1);
        })
        .write_all(asm.as_bytes())
        .unwrap_or_else(|err| {
            println!("cannot write file: {}", err);
            process::exit(1);
        });
    println!("write asm to {}", &filename);
}
