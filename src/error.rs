use std::fmt::Display;

const EVAL_APPEND: &str = "\n\nAborting evaluation due to previous error.";

#[derive(Debug)]
pub enum Error {
    ConversionError(String),
    DecimalConversionError(rust_decimal::Error),
    FactorialFloatNotSupportedError,
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
            Error::FactorialFloatNotSupportedError => {
                write!(f, "Calculating factorial for number with decimal place is currently not supported{}", EVAL_APPEND)
            }
            Error::OperationError => write!(
                f,
                "Operation error: Value overflowed or underflowed while calculating{}",
                EVAL_APPEND
            ),
        }
    }
}
