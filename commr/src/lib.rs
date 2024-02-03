use crate::Column::*;
use clap::{App, Arg};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

#[derive(Debug)]
enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("commr")
        .author("naoyuki miyata(n.miyata080825@gmail.com")
        .version("0.1.0")
        .about("rust comm")
        .arg(
            Arg::with_name("file1")
                .takes_value(true)
                .multiple(false)
                .required(true),
        )
        .arg(
            Arg::with_name("file2")
                .takes_value(true)
                .multiple(false)
                .required(false)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("col1")
                .short("1")
                .help("suppress printing of column 1")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("col2")
                .short("2")
                .help("suppress printing of column 2")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("col3")
                .short("3")
                .help("suppress printing of column 3")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("insensitive")
                .help("case insensitive comparison of line")
                .short("i")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIM")
                .help("use DELIM instead of TAB for field delimiter")
                .short("d")
                .long("output-delimiter")
                .default_value("\t")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        file1: matches.value_of_lossy("file1").unwrap().to_string(),
        file2: matches.value_of_lossy("file2").unwrap().to_string(),
        show_col1: matches.is_present("col1"),
        show_col2: matches.is_present("col2"),
        show_col3: matches.is_present("col3"),
        insensitive: matches.is_present("insensitive"),
        delimiter: matches.value_of_lossy("delimiter").unwrap().to_string(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;
    if file1 == "-" && file2 == "-" {
        return Err(From::from("both input files cannnot be STDIN (\"-\")"));
    }
    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };
    let print = |column: Column| {
        let mut columns = vec![];
        match column {
            Col1(val) => {
                if config.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if config.show_col1 {
                    columns.push("");
                }
                columns.push(val);
            }
            Col3(val) => {
                if config.show_col1 {
                    columns.push("");
                }
                if config.show_col2 {
                    columns.push("");
                }
                columns.push(val);
            }
        }
        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };
    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);
    let mut line1 = lines1.next();
    let mut line2 = lines2.next();
    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Equal => {
                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                Greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val), None) => {
                println!("{}", val);
                line1 = lines1.next();
            }
            (None, Some(val)) => {
                println!("{}", val);
                line2 = lines2.next();
            }
            _ => (),
        };
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
