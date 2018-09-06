//! Deals with strings (`String`)

use super::{Pass, PassSequence};
use color;
use colored::*;
use config::Config;
use unicode_segmentation::UnicodeSegmentation;

/// Run all passes with this `String`.
pub fn run(string: &String, config: &Config) {
    color::found(string, "string");
    pass_sequence!(string, config; structure, graphemes, words, bytes, modifications);
}

/// Print information about structure of this `String`.
fn structure(string: &String) {
    info!("ascii?", string.is_ascii());
}

/// Print information about graphemes of this `String`.
fn graphemes(string: &String) {
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(string.as_str(), true).collect();

    info!("array", format!("{:?}", graphemes));
    info!("len", graphemes.len());
}

/// Print information about words of this `String`.
fn words(string: &String) {
    let words: Vec<&str> = UnicodeSegmentation::unicode_words(string.as_str()).collect();

    info!("array", format!("{:?}", words));
    info!("len", words.len());
}

/// Print information about bytes of this `String`.
fn bytes(string: &String) {
    let bytes = string.as_bytes();

    info!("array", format!("{:?}", bytes));
    info!("len", string.len());
}

/// Print information about modifications of this `String`.
fn modifications(string: &String) {
    let upper = string.to_uppercase();
    let lower = string.to_lowercase();

    info!("upper", upper);
    info!("lower", lower);
}
