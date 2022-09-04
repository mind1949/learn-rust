use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Jurassic lianjie1949@gmail.com")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap_or_default();

    let lines = match matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|err| format!("illegal line count -- {}", err))?
    {
        Some(lines) => lines,
        _ => 10,
    };

    let bytes: Option<usize> = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|err| format!("illegal byte count -- {}", err))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => {
            let reader = std::io::stdin();
            let reader = BufReader::new(reader);
            let reader = Box::new(reader);
            Ok(reader)
        }
        _ => {
            let reader = File::open(filename)?;
            let reader = BufReader::new(reader);
            let reader = Box::new(reader);
            Ok(reader)
        }
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let num_file = config.files.len();
    for (file_idx, filename) in config.files.into_iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut reader) => {
                if num_file > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_idx > 0 { "\n" } else { "" },
                        filename,
                    );
                }

                match config.bytes {
                    None => {
                        let mut line = String::new();
                        for _ in 0..config.lines {
                            let bytes = reader.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                            print!("{}", line);
                            line.clear();
                        }
                    }
                    Some(num_bytes) => {
                        let mut handle = reader.take(num_bytes as u64);
                        let mut buffer = vec![0; num_bytes];
                        let bytes_read = handle.read(&mut buffer)?;
                        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]))
                    }
                }
            }
        };
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    let res = str::parse::<usize>(val);
    match res {
        Ok(u) if u > 0 => Ok(u),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo");

    //  A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0");
}
