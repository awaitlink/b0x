//! Contains utilities for storing configuration

/// Stores configuration
pub struct Config {
    input: String,
    ignored_passes: String,
}

impl Config {
    /// Creates a new configuration
    pub fn new(input: &str, ignored_passes: &str) -> Self {
        Self {
            input: String::from(input),
            ignored_passes: String::from(ignored_passes),
        }
    }

    /// Returns `true` if the pass is ignored in this `Config`
    pub fn is_ignored(&self, pass: &str) -> bool {
        self.ignored_passes
            .contains(&pass.chars().nth(0).unwrap().to_string())
    }

    /// Returns the `input` stored in this `Config`
    pub fn input(&self) -> &String {
        &self.input
    }
}
