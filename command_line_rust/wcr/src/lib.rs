use clap::{App, Arg};
use std::error::Error;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
    .version("0.1.0")
    .author("Jurassic lianjie1949@gmail.com")
    .about("Rust wc")
    .arg(
        Arg::with_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-")
    )
    .arg(
        Arg::with_name("lines")
        .short("l")
        .long("lines")
        .help("The number of lines in each input file is written to the standard output.")
        .takes_value(false)
    )
    .arg(
        Arg::with_name("words")
        .short("w")
        .long("words")
        .help(" The number of words in each input file is written to the standard output.")
        .takes_value(false)
    )
    .arg(
        Arg::with_name("bytes")
        .short("c")
        .long("bytes")
        .help("The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option.")
        .takes_value(false)
    )
    .arg(
        Arg::with_name("chars")
        .short("m")
        .long("chars")
        .conflicts_with("bytes")
        .help("The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte
        characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.")
        .takes_value(false)
    )
    .get_matches();

    let mut lines: bool = matches.is_present("lines");
    let mut words: bool = matches.is_present("words");
    let mut bytes: bool = matches.is_present("bytes");
    let chars: bool = matches.is_present("chars");

    // The default behavior will be to print lines, words, and bytes from STDIN,
    // which means those values in the configuration should be true when none have been explicitly requested by the user
    if [lines, words, bytes, chars].iter().all(|v| !v) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("FILE").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn count<T: BufRead>(mut reader: T) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += bytes_read;
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut total = FileInfo {
        num_lines: 0,
        num_bytes: 0,
        num_chars: 0,
        num_words: 0,
    };
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {
                if let Ok(info) = count(reader) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                    total.num_lines += info.num_lines;
                    total.num_words += info.num_words;
                    total.num_bytes += info.num_bytes;
                    total.num_chars += info.num_chars;
                };
            }
        }
    }
    let num_files = config.files.len();
    if num_files > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total.num_lines, config.lines),
            format_field(total.num_words, config.words),
            format_field(total.num_bytes, config.bytes),
            format_field(total.num_chars, config.chars)
        );
    }
    Ok(())
}

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;
    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
