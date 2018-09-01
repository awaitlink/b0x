//! Deals with strings (`String`)

use super::{Pass, PassSequence};
use color;
use colored::*;
use config::Config;

/// Run all passes with this `String`.
pub fn run(string: &String, config: &Config) {
    color::found(string, "string");
    pass_sequence!(string, config; info);
}

/// Print info about this `String`.
fn info(string: &String) {
    let len = string.len();
    let bytes = format!("{:?}", string.as_bytes());
    let is_ascii = string.is_ascii();
    let upper = string.to_uppercase();
    let lower = string.to_lowercase();

    info!("len(bytes)", len);
    info!("bytes", bytes);
    info!("ascii?", is_ascii);
    info!("upper", upper);
    info!("lower", lower);
}
