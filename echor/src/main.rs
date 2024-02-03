// https://crates.io/crates/clap
use clap::{App, Arg};

/// run command
/// ```bash
/// cargo run --quiet -- -n hello world
/// cargo run --quiet -- --help
/// cargo run --quiet -- --version
/// ```
fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Naoyuki Miiyata")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
