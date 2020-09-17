use anyhow::Result;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub struct SourceIter {
    underlying: std::vec::IntoIter<String>,
}

impl SourceIter {
    pub fn new(filenames: Vec<String>) -> SourceIter {
        SourceIter {
            underlying: filenames.into_iter(),
        }
    }
}

impl Iterator for SourceIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.underlying.next().map(|name| read_source(&name))
    }
}

pub fn read_sources(arg: &str) -> Result<SourceIter> {
    let metadata = fs::metadata(arg)?;

    let filenames = if metadata.is_dir() {
        fs::read_dir(arg)?
            .into_iter()
            .flat_map(|entry| {
                let filename = entry
                    .unwrap_or_else(|err| {
                        println!("cannot read file: {}", err);
                        process::exit(1);
                    })
                    .path()
                    .to_string_lossy()
                    .into_owned();

                vm_file(&filename)
            })
            .collect()
    } else {
        if !arg.ends_with(".vm") {
            println!("invalid filename, exptected to '*.vm': {:?}", arg);
            process::exit(1);
        }
        vec![String::from(arg)]
    };

    Ok(SourceIter::new(filenames))
}

fn vm_file(filename: &str) -> Option<String> {
    if filename.ends_with(".vm") {
        Some(String::from(filename))
    } else {
        None
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
