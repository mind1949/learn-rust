use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::{error::Error, vec};
#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum EntryType {
    Dir,
    File,
    Link,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Jurassic lianjie1949@gimal.com")
        .about("Rust find")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("NAME")
                .default_value(".")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Entry type")
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let names = matches
        .values_of_lossy("name")
        .map(|v| {
            v.iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();
    let entry_types = matches
        .values_of_lossy("type")
        .map(|entry_types| {
            entry_types
                .iter()
                .map(|entry_type| match entry_type.as_str() {
                    "f" => File,
                    "d" => Dir,
                    "l" => Link,
                    _ => {
                        unreachable!("Invalid type")
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(Config {
        paths: matches.values_of_lossy("path").unwrap_or_default(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut entry_types = config.entry_types;
    if entry_types.len() == 0 {
        entry_types = vec![Dir, File, Link]
    }

    let mut output: Vec<String> = Vec::new();
    for path in config.paths {
        let walker = walkdir::WalkDir::new(path).into_iter();
        for entry in walker {
            match entry {
                Err(err) => eprintln!("{}", err),
                Ok(entry) => {
                    let metadata = entry.metadata()?;
                    let mut has_type = false;
                    for entry_type in entry_types.iter() {
                        match entry_type {
                            Dir => {
                                if metadata.is_dir() {
                                    has_type = true
                                }
                            }
                            File => {
                                if metadata.is_file() {
                                    has_type = true;
                                }
                            }
                            Link => {
                                if metadata.is_symlink() {
                                    has_type = true;
                                }
                            }
                        }
                    }
                    if !has_type {
                        continue;
                    }

                    for name in config.names.iter() {
                        if let Some(filename) = entry.file_name().to_str() {
                            if name.is_match(filename) {
                                let path = entry.path().display().to_string();
                                output.push(path);
                            }
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
    output.sort();
    for path in output {
        println!("{}", path)
    }
    Ok(())
}
