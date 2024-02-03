use crate::TakeValue::*;
use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .author("naoyuki miyata(n.miyata080825@gmail.com")
        .version("0.1.0")
        .about("rust tail")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("input file(s)")
                .multiple(true)
                .takes_value(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("byte count")
                .takes_value(true)
                .multiple(false),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .help("line count")
                .takes_value(true)
                .multiple(false)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .takes_value(false)
                .multiple(false),
        )
        .get_matches();

    let parse_positive_int = |val: &str| -> MyResult<i64> {
        match val.parse::<i64>() {
            Ok(n) if n > 0 => Ok(n),
            _ => Err(From::from(val)),
        }
    };
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal format line count: {}", e))?
        .map_or(PlusZero, |val| TakeNum(val));

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal format line count: {}", e))?
        .map_or(None, |val| Some(TakeNum(val)));

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines,
        bytes: bytes,
        quiet: matches.is_present("quiet"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
