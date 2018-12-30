use b0x::config::Config;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use colored::*;
use termion::terminal_size;

const INPUT_HELP: &'static str =
    "Input data. Supported formats:
    • IP addresses
        ◦ IPv4: 1.1.1.1
        ◦ IPv6: 2606:4700:4700::1111
    • Unsigned integers
        ◦ Binary:      0b101010
        ◦ Octal:       0o52
        ◦ Hexadecimal: 0x2A
        ◦ Decimal:     42
    • Strings";

const IGNORE_PASSES_HELP: &'static str =
    "Ignore pass(-es). Specify first characters of the passes you want to ignore. For example:
    -pipe       | ignores `is_prime`, `power`, `english`
    --ignore sm | ignores `structure`, `modifications`";

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .help(INPUT_HELP)
                .multiple(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("passes")
                .help(IGNORE_PASSES_HELP)
                .short("p")
                .long("ignore")
                .takes_value(true)
                .use_delimiter(true),
        )
        .get_matches();

    let inputs = matches.values_of("input").unwrap();
    let ignored_passes = matches.value_of("passes").unwrap_or("");

    let terminal_width = terminal_size().unwrap_or((80, 24)).0;
    let length = inputs.len();
    let mut i = 1;

    for input in inputs {
        let config = Config::new(input, ignored_passes);
        b0x::run(config);

        if i < length {
            println!("{}", "-".repeat(terminal_width as usize).white().bold())
        }

        i += 1;
    }
}
