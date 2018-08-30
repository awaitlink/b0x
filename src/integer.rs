//! Deals with unsigned integers (`u128`)

use super::color;

/// Print everything about this `u128`.
pub fn main(integer: u128) {
    color::found(&integer.to_string(), "u128");

    print_radix(integer);
    print_misc(integer);
}

/// Print this `u128` in different radixes.
fn print_radix(integer: u128) {
    let bin = format!("{:b}", integer);
    let oct = format!("{:o}", integer);
    let dec = integer.to_string();
    let hex = format!("{:x}", integer);
    let hex_upper = format!("{:X}", integer);

    let ones = integer.count_ones().to_string();
    let zeros = integer.count_zeros().to_string();
    let leading_zeros = integer.leading_zeros().to_string();
    let trailing_zeros = integer.trailing_zeros().to_string();

    color::print("bin", &bin);
    color::print("  ├ ones", &ones);
    color::print("  ├ zeros", &zeros);
    color::print("  ├ leading zeros", &leading_zeros);
    color::print("  └ trailing zeros", &trailing_zeros);
    color::print("oct", &oct);
    color::print("dec", &dec);
    color::print("hex", &hex);
    color::print("HEX", &hex_upper);
}

/// Print misc info about this `u128`.
fn print_misc(integer: u128) {
    let next = match integer.checked_next_power_of_two() {
        Some(v) => v.to_string(),
        None => String::from("<error>"),
    };

    color::print("next 2^x", &next);
}
