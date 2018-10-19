#[macro_use]
extern crate clap;
extern crate b0x;

use b0x::config::Config;
use clap::{App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .help(
                    "Input data. Supported formats:
• String
• Integer
    • Decimal:     42
    • Binary:      0b101010
    • Octal:       0o52
    • Hexadecimal: 0x2A",
                ).required(true)
                .index(1),
        ).arg(
            Arg::with_name("ignored-passes")
                .help("Ignore pass(-es)")
                .short("p")
                .long("ignore")
                .multiple(true)
                .takes_value(true)
                .possible_values(&[
                    // integer
                    "radix",
                    "prime",
                    "power",
                    "english",
                    // string
                    "structure",
                    "graphemes",
                    "words",
                    "bytes",
                    "modifications",
                ]).use_delimiter(true),
        ).get_matches();

    let input = matches.value_of("input").unwrap();
    let ignored_passes = match matches.values_of("ignored-passes") {
        Some(v) => v.map(|e| String::from(e)).collect(),
        None => Vec::new(),
    };

    let config = Config::new(String::from(input), ignored_passes);
    b0x::run(config);
}
