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
        Ok(match self {
            Self::Number(n) | Self::Float(n) => {
                Decimal::from_str(&*n).map_err(Error::DecimalConversionError)?
            }
            Self::Neg(n) => -n.eval()?,
            Self::Add(a, b) => a
                .eval()?
                .checked_add(b.eval()?)
                .ok_or(Error::OperationError)?,
            Self::Sub(a, b) => a
                .eval()?
                .checked_sub(b.eval()?)
                .ok_or(Error::OperationError)?,
            Self::Mul(a, b) => a
                .eval()?
                .checked_mul(b.eval()?)
                .ok_or(Error::OperationError)?,
            Self::Div(a, b) => a
                .eval()?
                .checked_div(b.eval()?)
                .ok_or(Error::OperationError)?,
            Self::Mod(a, b) => a
                .eval()?
                .checked_rem(b.eval()?)
                .ok_or(Error::OperationError)?,
            Self::Pow(a, b) => a
                .eval()?
                .checked_powd(b.eval()?)
                .ok_or(Error::OperationError)?,
            Self::Factorial(n) => factorial((**n).eval()?)?,
        })
    }
}
