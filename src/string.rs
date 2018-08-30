//! Deals with strings (`&str`)

use super::color;

/// Print everything about this `&str`.
pub fn main(string: &str) {
    color::found(string, "string");

    print_data(string);
}

/// Print data about this `&str`.
fn print_data(string: &str) {
    let len = string.len().to_string();
    let bytes = format!("{:?}", string.as_bytes());
    let is_ascii = string.is_ascii().to_string();
    let upper = string.to_uppercase();
    let lower = string.to_lowercase();

    color::print("len", &len);
    color::print("bytes", &bytes);
    color::print("ascii?", &is_ascii);
    color::print("upper", &upper);
    color::print("lower", &lower);
}
