//! Deals with unsigned integers (`u128`)

use super::*;
use english_numbers::{convert, Formatting};
use primal::as_perfect_power;

/// Pass sequence for unsigned integers (`u128`).
pub struct IntegerPasses;
impl<'a> PassSequence<'a> for IntegerPasses {
    type T = u128;
    const TY_NAME: &'static str = "u128";
    const PASSES: &'a [Pass<'a, Self::T>] =
        pass_sequence![radix, is_prime, power, modifications, english];
}

/// This `u128` in different radixes.
pub fn radix<'a>(integer: &u128) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();

    let bin = format!("{:b}", integer);
    let oct = format!("{:o}", integer);
    let dec = integer;
    let hex = format!("{:x}", integer);
    let hex_upper = format!("{:X}", integer);

    let ones = integer.count_ones();
    let zeros = integer.count_zeros();
    let leading_zeros = integer.leading_zeros();
    let trailing_zeros = integer.trailing_zeros();

    push!(result, "bin", bin);
    push!(result, "  ├ ones", ones);
    push!(result, "  ├ zeros", zeros);
    push!(result, "  ├ visible zeros", zeros - leading_zeros);
    push!(result, "  ├ leading zeros", leading_zeros);
    push!(result, "  └ trailing zeros", trailing_zeros);
    push!(result, "oct", oct);
    push!(result, "dec", dec);
    push!(result, "hex", hex);
    push!(result, "HEX", hex_upper);

    result
}

/// Is this `u128` is prime?
pub fn is_prime<'a>(integer: &u128) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();

    if *integer <= (u64::max_value() as u128) {
        push!(result, "prime?", primal::is_prime(*integer as u64));
    } else {
        push!(result, "prime?", n / a);
    }

    result
}

/// Information about this `u128` related to powers.
pub fn power<'a>(integer: &u128) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();

    if *integer <= (u64::max_value() as u128) {
        let perfect_power = as_perfect_power(*integer as u64);
        push!(
            result,
            "perfect a^k",
            format!("{} ^ {}", perfect_power.0, perfect_power.1)
        );
    } else {
        push!(result, "perfect a^k", n / a);
    }

    let is_power_of_two = integer.is_power_of_two();
    push!(result, "2^k?", is_power_of_two);

    // If `integer` is already a power of two, find the actual next one.
    match (if is_power_of_two {
        integer.saturating_add(1)
    } else {
        *integer
    })
    .checked_next_power_of_two()
    {
        Some(v) => push!(result, "next 2^k", v),
        None => push!(result, "next 2^k", n / a),
    };

    result
}

/// Modifications of this `u128`.
pub fn modifications<'a>(integer: &u128) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();
    push!(result, "swap bytes", integer.swap_bytes());
    result
}

/// This `u128` in English.
pub fn english<'a>(integer: &u128) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();
    if *integer <= (i64::max_value() as u128) {
        let mut fmt = Formatting::all();
        fmt.title_case = false;

        push!(
            result,
            "english",
            format!("\"{}\"", convert(*integer as i64, fmt))
        );
    } else {
        push!(result, "english", n / a);
    }
    result
}
