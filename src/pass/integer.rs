//! Deals with unsigned integers (`u128`)

use super::*;
use crate::color;
use english_numbers::{convert, Formatting};
use primal::{as_perfect_power, is_prime};

/// Run all passes with this `u128`.
pub fn run(integer: u128, config: &Config) {
    color::found(&integer.to_string(), "u128");
    pass_sequence!(&integer, config; radix, prime, power, english);
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
    info!(2; "├ ones", ones);

    indent_println!(
        "{} {} ({})",
        "  ├ zeros".blue().bold(),
        (zeros - leading_zeros).to_string().green().bold(),
        zeros.to_string().green().bold()
    );

    info!(2; "├ leading zeros", leading_zeros);
    info!(2; "└ trailing zeros", trailing_zeros);
    info!("oct", oct);
    info!("dec", dec);
    info!("hex", hex);
    info!("HEX", hex_upper);
}

/// Print if this `u128` is prime.
fn prime(integer: &u128) {
    if *integer <= (u64::max_value() as u128) {
        info!("prime?", is_prime(*integer as u64));
    } else {
        na!("prime?");
    }
}

/// Print information about this `u128` related to powers.
fn power(integer: &u128) {
    if *integer <= (u64::max_value() as u128) {
        let perfect_power = as_perfect_power(*integer as u64);
        info!(
            "perfect a^k",
            format!("{} ^ {}", perfect_power.0, perfect_power.1)
        );
    } else {
        na!("perfect a^k");
    }

    let is_power_of_two = integer.is_power_of_two();
    info!("2^k?", is_power_of_two);

    // If `integer` is already a power of two, find the actual next one.
    match (if is_power_of_two {
        integer.saturating_add(1)
    } else {
        *integer
    })
    .checked_next_power_of_two()
    {
        Some(v) => info!("next 2^k", v),
        None => na!("next 2^k"),
    };
}

/// Print this `u128` in English.
fn english(integer: &u128) {
    if *integer <= (i64::max_value() as u128) {
        let mut fmt = Formatting::all();
        fmt.title_case = false;

        info!("english", format!("\"{}\"", convert(*integer as i64, fmt)));
    } else {
        na!("english");
    }
}
