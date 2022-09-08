use clap::{App, Arg};
use std::error::Error;
use std::{
    fs::File,
    io::{
        self, Write, {BufRead, BufReader},
    },
};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .author("Jurassic <lianjie1949@gmail.com>")
        .version("0.1.0")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("COUNT")
                .help("Show counts")
                .takes_value(false),
        )
        .get_matches();

    let in_file = matches.value_of("in_file").unwrap_or_default().to_string();
    let out_file = matches.value_of("out_file").map(|v| v.to_string());
    let count = matches.is_present("count");
    Ok(Config {
        in_file,
        out_file,
        count: count,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match config.out_file {
        Some(out_name) => {
            let file = File::create(out_name).unwrap();
            Box::new(file)
        }
        _ => Box::new(io::stdout()),
    };
    let mut print = |count: u32, text: &str| -> MyResult<()> {
        if config.count {
            write!(out_file, "{:>4} ", count)?;
        }
        write!(out_file, "{}", text)?;
        Ok(())
    };

    let mut previouse = "".to_string();
    let mut line = "".to_string();
    let bytes = file.read_line(&mut line)?;
    if bytes == 0 {
        return Ok(());
    }
    let mut count: u32 = 1;
    loop {
        previouse = line.clone();
        line.clear();
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            print(count, &previouse)?;
            break;
        }
        if previouse.trim_end() == line.trim_end() {
            count += 1;
        } else {
            print(count, &previouse)?;
            count = 1;
        }
    }

    Ok(())
}
