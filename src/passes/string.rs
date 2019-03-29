//! Deals with strings (`String`)

use super::*;
use unicode_segmentation::UnicodeSegmentation;

/// Pass sequence for strings (`String`).
pub struct StringPasses;
impl<'a> PassSequence<'a> for StringPasses {
    type T = String;
    const TY_NAME: &'static str = "string";
    const PASSES: &'a [Pass<'a, Self::T>] =
        pass_sequence![structure, graphemes, words, bytes, modifications];
}

/// Information about structure of this `String`.
pub fn structure<'a>(string: &String) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();
    push!(result, "empty?", string.is_empty());
    push!(result, "ascii?", string.is_ascii());
    result
}

/// Information about graphemes of this `String`.
pub fn graphemes<'a>(string: &String) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();
    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(string.as_str(), true).collect();

    push!(result, "array", format!("{:?}", graphemes));
    push!(result, "len", graphemes.len());
    result
}

/// Information about words of this `String`.
pub fn words<'a>(string: &String) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();
    let words: Vec<&str> = UnicodeSegmentation::unicode_words(string.as_str()).collect();

    push!(result, "array", format!("{:?}", words));
    push!(result, "len", words.len());
    result
}

/// Information about bytes of this `String`.
pub fn bytes<'a>(string: &String) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();
    let bytes = string.as_bytes();

    push!(result, "array", format!("{:?}", bytes));
    push!(result, "len", string.len());
    result
}

/// Information about modifications of this `String`.
pub fn modifications<'a>(string: &String) -> Vec<(&'a str, Info)> {
    let mut result = Vec::new();
    let upper = string.to_uppercase();
    let lower = string.to_lowercase();

    push!(result, "upper", upper);
    push!(result, "lower", lower);
    result
}
