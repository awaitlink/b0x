#![deny(missing_docs)]

//! A simple CLI tool to display information about the provided input.
//!
//! See [`README.md`](https://github.com/u32i64/b0x#readme) for installation, usage, examples...

pub mod config;
pub mod pass;

use crate::config::Config;
use std::net::IpAddr;

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

macro_rules! try_prefix_seq {
    {$input:expr, $config:expr, $ending:expr; $prefix:expr, $radix:expr, $($tail:tt)*} => {
        if !try_prefix($input, $prefix, $radix, $config) {
            try_prefix_seq! { $input, $config, $ending; $($tail)* }
        }
    };

    {$input:expr, $config:expr, $ending:expr;} => {
        $ending
    };
}

/// Given a `Config`, try to parse the input stored in it as a number and print information.
/// If the input can't be parsed as a `u128`, fall back to String.
pub fn run(config: Config) {
    let input = config.input();
    let input_trim = input.trim();

    match input_trim.parse::<IpAddr>() {
        Ok(v) => pass::ip_addr::run(&v, &config),
        Err(_) => try_prefix_seq! {
            input_trim, &config, match input_trim.parse::<u128>() {
                Ok(v) => pass::integer::run(v, &config),
                Err(_) => pass::string::run(input, &config),
            };

            "0b", 2,
            "0o", 8,
            "0x", 16,
        },
    };
}
