#[macro_use]
extern crate clap;
extern crate b0x;

use clap::{App, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .help("Input data")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();

    b0x::run(input);
}
