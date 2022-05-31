use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
};

const EVAL_APPEND: &str = "\n\nAborting evaluation due to previous error.";

#[derive(Debug)]
pub enum Error {
    ConversionError(String),
    DecimalConversionError(rust_decimal::Error),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    OperationError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConversionError(s) => {
                write!(f, "Conversion error: {}{}", s, EVAL_APPEND)
            }
            Error::DecimalConversionError(e) => {
                write!(f, "Decimal conversion error: {}{}", e, EVAL_APPEND)
            }
            Error::ParseIntError(e) => {
                write!(f, "Parse int error: {}{}", e, EVAL_APPEND)
            }
            Error::ParseFloatError(e) => {
                write!(f, "Parse float error: {}{}", e, EVAL_APPEND)
            }
            Error::OperationError => write!(
                f,
                "Operation error: Value overflowed or underflowed while calculating{}",
                EVAL_APPEND
            ),
        }
    }
}
