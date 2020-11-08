use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Source {
    pub path: Box<Path>,
    pub content: String,
}

impl Source {
    fn basename(&self) -> Result<String> {
        self.path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .ok_or(anyhow!("invalid filename : {}", self.path.display()))
    }

    pub fn token_xml_filename(&self) -> Result<PathBuf> {
        let name = self.basename()?;
        let mut path = self.path.clone().to_path_buf();
        path.set_file_name(format!("{}.tokens.xml", name));
        return Ok(path);
    }

    pub fn ast_xml_filename(&self) -> Result<PathBuf> {
        let name = self.basename()?;
        let mut path = self.path.clone().to_path_buf();
        path.set_file_name(format!("{}.ast.xml", name));
        return Ok(path);
    }
}

pub struct SourceIter {
    underlying: std::vec::IntoIter<Box<Path>>,
}

impl SourceIter {
    pub fn new(filenames: Vec<Box<Path>>) -> SourceIter {
        SourceIter {
            underlying: filenames.into_iter(),
        }
    }
}

impl Iterator for SourceIter {
    type Item = Result<Source>;

    fn next(&mut self) -> Option<Result<Source>> {
        self.underlying.next().map(|path| read_source(path))
    }
}

pub fn read_sources(arg: &str) -> Result<SourceIter> {
    let filenames = get_filenames(&arg)?;
    Ok(SourceIter::new(filenames))
}

pub fn get_filenames(arg: &str) -> Result<Vec<Box<Path>>> {
    let path = Path::new(arg);

    if !path.exists() {
        return Err(anyhow!("no such path or filename: {}", arg));
    }

    let filenames = if path.is_dir() {
        collect_filenames(&path)?
    } else {
        if !arg.ends_with(".jack") {
            return Err(anyhow!(
                "invalid filename, exptected to '*.jack': {:?}",
                arg
            ));
        }

        vec![path.to_path_buf().into_boxed_path()]
    };

    Ok(filenames)
}

fn collect_filenames(path: &Path) -> Result<Vec<Box<Path>>> {
    let paths: Result<Vec<Box<Path>>, std::io::Error> = path
        .read_dir()?
        .filter(|entry| {
            entry.as_ref().map_or(false, |e| {
                e.path().extension().map_or(false, |ext| ext == "jack")
            })
        })
        .map(|entry| entry.map(|e| e.path().into_boxed_path()))
        .collect();

    paths.map_err(|err| anyhow!(err))
}

pub fn read_source(path: Box<Path>) -> Result<Source> {
    let mut content = String::new();
    File::open(&path)?.read_to_string(&mut content)?;
    Ok(Source { path, content })
}
