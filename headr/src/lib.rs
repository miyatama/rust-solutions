use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
        .author("Naoyuki Miyata(n.miyata080825@gmail.com")
        .about("rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .help("show line(s)")
                .takes_value(true)
                .value_name("LINES")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("number of bytes")
                .takes_value(true)
                .value_name("BYTES")
                .conflicts_with("lines"),
        )
        .get_matches();

    // Option::transpose
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose
    // Option<Result> -> Result<Option>
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illiegal bytes count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes: bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                    "{}==> {} <==",
                    if file_num > 0 { "\n"} else {""},
                    filename,
                    )
                }
                /*
                let mut contents = String::new();
                file.read_to_string(&mut contents); // Danger!! メモリ容量を超える可能性あり
                let bytes = contents.as_bytes();
                print!("{}", Stirng::from_utf8_lossy(&bytes[..num_bytes])); // Danger!! 範囲外を参照する可能性あり
                */

                if let Some(num_bytes) = config.bytes {
                    /*
                    let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
                    print!( "{}", String::from_utf8_lossy(bytes))
                     */
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read]),
                    )

                } else {
                let mut line = String::new();
                for _ in 0..config.lines {
                    let bytes = file.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }
                    print!("{}", line);
                    line.clear();
                }

                }
            },
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    /*
    Err(From::from(val)) or
    Err(val.into()) or
    Err(Into::into(val))
     */
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
