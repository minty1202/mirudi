use std::fmt;

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum DiffMode {
    Slice,
    Words,
    Lines,
    Chars,
}

impl fmt::Display for DiffMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            DiffMode::Slice => "slice",
            DiffMode::Words => "words",
            DiffMode::Lines => "lines",
            DiffMode::Chars => "chars",
        };
        write!(f, "{}", text)
    }
}
