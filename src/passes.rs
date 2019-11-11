//! Defines passes

use crate::config::Config;
use colored::*;

const INFO_INDENT: &str = "   ";

/// Information, value (what is displayed on the right).
pub enum Info {
    /// Available information.
    Available(String),
    /// Shows `n/a`.
    NotAvailable,
    /// Shows `other`.
    Other,
}

/// One pass that gives related information regarding `T`.
pub struct Pass<'a, T>(&'a str, &'a dyn Fn(&T) -> Vec<(&'a str, Info)>);

impl<'a, T> Pass<'a, T> {
    fn name(&self) -> &'a str {
        &self.0
    }

    fn run(&self, input: &T) -> Vec<(&'a str, Info)> {
        (self.1)(input)
    }
}

/// A sequence of passes. This is what runs for a given value.
pub trait PassSequence<'a> {
    /// Type of the value that this sequence can give information about.
    type T: 'a;
    /// Name of `T` which will be displayed in the beginning of output via `found TY_NAME(value)`.
    const TY_NAME: &'static str;
    /// Passes which this sequence consists of.
    const PASSES: &'a [Pass<'a, Self::T>];

    /// Runs all passes in the sequence that are not ignored and returns the combined result.
    fn run(input: &Self::T, config: &Config) -> Vec<(&'a str, Result<Vec<(&'a str, Info)>, ()>)> {
        let mut results = Vec::new();
        for pass in Self::PASSES {
            results.push((
                pass.name(),
                if !config.is_ignored(&pass.name()) {
                    Ok(pass.run(input))
                } else {
                    Err(())
                },
            ));
        }

        results
    }
}

/// Runs a pass sequence for a given input, outputting all received information.
pub fn run<'a, S>(input: &S::T, config: &Config)
where
    S: PassSequence<'a>,
    S::T: ToString + 'a,
{
    let data = input.to_string().red().bold();
    let ty = S::TY_NAME.yellow().bold();

    println!("found {}({})", ty, data);

    for (name, result) in S::run(input, config) {
        println!(
            "{} {} {}",
            match result {
                Ok(_) => "➔",
                Err(_) => "✘",
            },
            name.magenta().bold(),
            match result {
                Ok(_) => "",
                Err(_) => "ignored",
            },
        );

        if let Ok(result) = result {
            for (name, info) in result {
                println!(
                    "{}{} {}",
                    INFO_INDENT,
                    name.to_string().blue().bold(),
                    match info {
                        Info::Available(value) => value.to_string().green().bold(),
                        Info::NotAvailable => "n/a".cyan().bold(),
                        Info::Other => "other".cyan().bold(),
                    }
                );
            }
        }
    }
}

macro_rules! push {
    ($result:ident, $name:expr, n / a) => {
        $result.push(($name, $crate::passes::Info::NotAvailable));
    };

    ($result:ident, $name:expr, other) => {
        $result.push(($name, $crate::passes::Info::Other));
    };

    ($result:ident, $name:expr, $value:expr) => {
        $result.push(($name, $crate::passes::Info::Available($value.to_string())));
    };
}

macro_rules! pass_sequence {
    ($($id:expr),*) => {
        &[$($crate::passes::Pass(stringify!($id), &$id)),*]
    }
}

pub mod integer;
pub mod ip_addr;
pub mod string;
