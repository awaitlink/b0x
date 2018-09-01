//! A simple CLI tool to display information about the provided input.
//!
//! # Installation
//! ```console
//! $ cargo install b0x
//! ```
//!
//! # Usage
//! While you don't see it below, it prints everything in color.
//!
//! #### Numbers
//! Supported formats:
//! - Decimal: `42`
//! - Binary: `0b101010`
//! - Octal: `0o52`
//! - Hexadecimal: `0x2A`
//!
//! ```console
//! $ b0x 0xC0FFEE
//! found u128(12648430)
//! --> radix
//! bin 110000001111111111101110
//!   ├ ones 16
//!   ├ zeros 8 (112)
//!   ├ leading zeros 104
//!   └ trailing zeros 1
//! oct 60177756
//! dec 12648430
//! hex c0ffee
//! HEX C0FFEE
//! --> misc
//! next 2^x 16777216
//! ```
//!
//! #### Strings
//!
//! ```console
//! $ b0x "TeSt StRiNg"
//! found string(TeSt StRiNg)
//! --> structure
//! ascii? true
//! --> graphemes
//!  ["T", "e", "S", "t", " ", "S", "t", "R", "i", "N", "g"]
//! len 11
//! --> words
//!  ["TeSt", "StRiNg"]
//! len 2
//! --> bytes
//!  [84, 101, 83, 116, 32, 83, 116, 82, 105, 78, 103]
//! len 11
//! --> modifications
//! upper TEST STRING
//! lower test string
//! ```

extern crate colored;
extern crate unicode_segmentation;

mod color;
pub mod pass;
pub mod config;

use config::Config;

/// Try to parse a number which has a prefix which determines the radix,
/// and print information if successful.
fn try_prefix(input: &str, prefix: &str, radix: u32, config: &Config) -> bool {
    if input.starts_with(prefix) {
        let clean_input = input.replacen(prefix, "", 1);
        if let Ok(integer) = u128::from_str_radix(&clean_input, radix) {
            pass::integer::run(integer, config);
            return true;
        }
    }

    false
}

/// Given a `Config`, try to parse the input stored in it as a number and print information.
/// If the input can't be parsed as a `u128`, fall back to String.
pub fn run(config: Config) {
    let input = config.input();
    let input_trim = input.trim();

    if !try_prefix(input_trim, "0b", 2, &config) {
        if !try_prefix(input_trim, "0o", 8, &config) {
            if !try_prefix(input_trim, "0x", 16, &config) {
                match input_trim.parse::<u128>() {
                    Ok(v) => pass::integer::run(v, &config),
                    Err(_) => pass::string::run(input, &config),
                }
            }
        }
    }
}
