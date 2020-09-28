pub mod codegen;
pub mod parser;
pub mod source;

use crate::codegen::*;
use crate::parser::*;
use crate::source::*;

use std::env;
use std::process;

use std::fs::File;
use std::io::Write;
use std::path::Path;

/**
 * 1. Read file or directory
 * 2. parse each vm files to VMCommand(s)
 * 3. generate hack asm from VMCommand(s)
 *
 */
pub fn process() {
    // get filename or directory from args
    let arg = get_first_arg();
    let (dir, vm_name) = parse_arg(&arg);

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
    let (asm, errors) = generate(results);

    if !errors.is_empty() {
        println!("codegen error: ");
        errors.into_iter().for_each(|err| println!("  {:?}", err));
        process::exit(1);
    }

    // write to file
    write_asm(&dir, &vm_name, &asm);

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

fn parse_arg(arg: &str) -> (String, String) {
    let path = Path::new(arg);

    if !path.exists() {
        println!("no such path or filename: {}", arg);
        process::exit(1);
    }

    let basename = path
        .file_name()
        .unwrap_or_else(|| {
            println!("invalid filename: {}", arg);
            process::exit(1);
        })
        .to_string_lossy()
        .into_owned();
    let dir = if path.is_dir() {
        path
    } else {
        path.parent().unwrap_or_else(|| {
            println!("invalid filename: {}", arg);
            process::exit(1);
        })
    };

    let vm_name = if path.is_dir() {
        basename
    } else {
        if basename.ends_with(".vm") {
            basename.replace(".vm", "")
        } else {
            println!("filename is expected to be ends with .vm: {}", arg);
            process::exit(1);
        }
    };

    (dir.to_string_lossy().into_owned(), vm_name)
}

fn write_asm(dir: &str, vm_name: &str, asm: &str) {
    let path = Path::new(dir);
    let filename = format!("{}.asm", vm_name);
    let filename = path.join(filename);

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
    println!("write asm to {}", &filename.to_string_lossy());
}
