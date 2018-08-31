//! Contains utilities for storing configuration

/// Stores configuration
pub struct Config {
    input: String,
    ignored_passes: Vec<String>,
}

impl Config {
    /// Creates a new configuration
    pub fn new(input: String, ignored_passes: Vec<String>) -> Self {
        Self {
            input,
            ignored_passes,
        }
    }

    /// Returns `true` if the pass is ignored in this `Config`
    pub fn is_ignored(&self, pass: &str) -> bool {
        self.ignored_passes.contains(&String::from(pass))
    }

    /// Returns `the input stored in this `Config`
    pub fn input(&self) -> &String {
        &self.input
    }
}