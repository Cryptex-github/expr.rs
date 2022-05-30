use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

use crate::error::Error;

pub fn factorial(d: &Decimal) -> Result<Decimal, Error> {
    if d.is_zero() {
        Ok(Decimal::from(1_u8))
    } else {
        Ok(Decimal::from_u128(
            (1..=d.to_u128().ok_or_else(|| Error::ConversionError(
                "Failed to convert Decimal to u128".to_string(),
            ))?)
                .product::<u128>() as u128,
        )
        .ok_or_else(|| Error::ConversionError(
            "Failed to convert u128 to Decimal".to_string(),
        ))?)
    }
}
