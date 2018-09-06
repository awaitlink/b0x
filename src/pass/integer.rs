//! Deals with unsigned integers (`u128`)

use super::{Pass, PassSequence};
use color;
use colored::*;
use config::Config;

/// Run all passes with this `u128`.
pub fn run(integer: u128, config: &Config) {
    color::found(&integer.to_string(), "u128");
    pass_sequence!(&integer, config; radix, misc);
}

/// Print this `u128` in different radixes.
fn radix(integer: &u128) {
    let bin = format!("{:b}", integer);
    let oct = format!("{:o}", integer);
    let dec = integer;
    let hex = format!("{:x}", integer);
    let hex_upper = format!("{:X}", integer);

    let ones = integer.count_ones();
    let zeros = integer.count_zeros();
    let leading_zeros = integer.leading_zeros();
    let trailing_zeros = integer.trailing_zeros();

    info!("bin", bin);
    info!("  ├ ones", ones);

    println!(
        "   {} {} ({})",
        "  ├ zeros".blue().bold(),
        (zeros - leading_zeros).to_string().green().bold(),
        zeros.to_string().green().bold()
    );

    info!("  ├ leading zeros", leading_zeros);
    info!("  └ trailing zeros", trailing_zeros);
    info!("oct", oct);
    info!("dec", dec);
    info!("hex", hex);
    info!("HEX", hex_upper);
}

/// Print misc information about this `u128`.
fn misc(integer: &u128) {
    let next = match integer.checked_next_power_of_two() {
        Some(v) => v.to_string().green().bold(),
        None => String::from("n/a").cyan().bold(),
    };

    println!("   {} {}", "next 2^x".blue().bold(), next);
}
