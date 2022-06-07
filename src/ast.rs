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

macro_rules! operation {
    ($a:ident, $b:ident, $op:ident) => {
        $a
            .eval()?
            .$op($b.eval()?)
            .ok_or(Error::OperationError)?
    };
}

impl Token {
    pub fn eval(&self) -> Result<Decimal, Error> {
        Ok(match self {
            Self::Number(n) | Self::Float(n) => {
                Decimal::from_str(&*n).map_err(Error::DecimalConversionError)?
            }
            Self::Neg(n) => -n.eval()?,
            Self::Add(a, b) => operation!(a, b, checked_add),
            Self::Sub(a, b) => operation!(a, b, checked_sub),
            Self::Mul(a, b) => operation!(a, b, checked_mul),
            Self::Div(a, b) => operation!(a, b, checked_div),
            Self::Mod(a, b) => operation!(a, b, checked_rem),
            Self::Pow(a, b) => operation!(a, b, checked_powd),
            Self::Factorial(n) => factorial((**n).eval()?)?,
        })
    }
}
