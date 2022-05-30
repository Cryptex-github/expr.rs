use std::{num::{ParseFloatError, ParseIntError}, fmt::Display};

const EVAL_PREPEND: &str = "Evaluation error:\n\n";
const EVAL_APPEND: &str = "\n\nAborting evaluation due to previous error.";

#[derive(Debug)]
pub enum Error {
    ConversionError(String),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConversionError(s) => write!(f, "{}Conversion error: {}{}", EVAL_PREPEND, s, EVAL_APPEND),
            Error::ParseIntError(e) => write!(f, "{}Parse int error: {}{}", EVAL_PREPEND, e, EVAL_APPEND),
            Error::ParseFloatError(e) => write!(f, "{}Parse float error: {}{}", EVAL_PREPEND, e, EVAL_APPEND),
        }
    }
}
