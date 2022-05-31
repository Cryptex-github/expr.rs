use rust_decimal::{prelude::ToPrimitive, Decimal};

use crate::error::Error;

pub fn factorial(d: &Decimal) -> Result<Decimal, Error> {
    let n = d.to_u8().ok_or_else(
        || Error::ConversionError(
            "Failed to convert Decimal to u8 while calculating factorial, this is done to minimize the chance of overflow"
            .to_string()
    ))?;

    if n <= 1 {
        Ok(Decimal::from(1_u8))
    } else if n == 2 {
        Ok(Decimal::from(2_u8))
    } else {
        let mut res = Decimal::from(1_u8);

        for i in 1..=n {
            res = res
                .checked_mul(Decimal::from(i))
                .ok_or(Error::OperationError)?;
        }

        Ok(res)
    }
}
