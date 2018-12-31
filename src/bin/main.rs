use b0x::config::Config;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use colored::*;

fn input_help() -> String {
    format!(
        "Input data. Supported formats:
    • IP addresses
        ◦ IPv4: {}
        ◦ IPv6: {}
    • Unsigned integers
        ◦ Binary:      {}
        ◦ Octal:       {}
        ◦ Hexadecimal: {}
        ◦ Decimal:     {}
    • Strings",
        "127.0.0.1".blue(),
        "2606:4700:4700::1111".blue(),
        "0b101010".blue(),
        "0o52".blue(),
        "0x2A".blue(),
        "42".blue()
    )
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .settings(&[AppSettings::UnifiedHelpMessage, AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("input")
                .help(&input_help())
                .multiple(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("passes")
                .help(
                    "Ignores pass(-es). Specify first characters of the passes you want to ignore",
                )
                .short("p")
                .long("ignore")
                .takes_value(true)
                .use_delimiter(true),
        )
        .get_matches();

    let inputs = matches.values_of("input").unwrap();
    let ignored_passes = matches.value_of("passes").unwrap_or("");

    let terminal_width = term_size::dimensions().unwrap_or((80, 24)).0;
    let length = inputs.len();
    let mut i = 1;

    for input in inputs {
        let config = Config::new(input, ignored_passes);
        b0x::run(config);

        if i < length {
            println!("{}", "-".repeat(terminal_width))
        }

        i += 1;
    }
}
