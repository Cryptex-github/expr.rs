use rust_decimal::prelude::*;

use crate::error::Error;
use crate::formulas::factorial;

#[derive(Debug, Clone)]
pub enum Token {
    Number(String),
    Float(String),
    Neg(Box<Token>),
    Add(Box<Token>, Box<Token>),
    Sub(Box<Token>, Box<Token>),
    Mul(Box<Token>, Box<Token>),
    Div(Box<Token>, Box<Token>),
    Mod(Box<Token>, Box<Token>),
    Pow(Box<Token>, Box<Token>),
    Factorial(Box<Token>), // Left value
}

impl Token {
    pub fn eval(&self) -> Result<Decimal, Error> {
        match self {
            Self::Number(n) => Ok(Decimal::from_i64(
                (*n).parse::<i64>().map_err(Error::ParseIntError)?,
            )
            .ok_or_else(|| Error::ConversionError(
                "Cannot convert to Decimal".to_string(),
            ))?),
            Self::Float(n) => Ok(Decimal::from_f64(
                (*n).parse::<f64>().map_err(Error::ParseFloatError)?,
            )
            .ok_or_else(|| Error::ConversionError(
                "Cannot convert to Decimal".to_string(),
            ))?),
            Self::Neg(n) => Ok(-n.eval()?),
            Self::Add(a, b) => Ok(a.eval()? + b.eval()?),
            Self::Sub(a, b) => Ok(a.eval()? - b.eval()?),
            Self::Mul(a, b) => Ok(a.eval()? * b.eval()?),
            Self::Div(a, b) => Ok(a.eval()? / b.eval()?),
            Self::Mod(a, b) => Ok(a.eval()? % b.eval()?),
            Self::Pow(a, b) => Ok(a.eval()?.powd(b.eval()?)),
            Self::Factorial(n) => Ok(factorial(&(**n).eval()?)?),
        }
    }
}
