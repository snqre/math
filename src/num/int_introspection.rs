use crate::num::int;
use ::num_traits as num;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::thiserror::Error)]
#[repr(u8)]
pub enum Error {
    #[error("")]
    UnsupportedConversion,
    #[error("")]
    ConversionOverflow,
    #[error("")]
    ConversionUnderflow
}

pub trait IntIntrospection: int::Int {

    fn is_unsigned(&self) -> bool {
        !self.is_signed()
    }

    fn is_signed(&self) -> bool {
        Self::min_value() < Self::zero()
    }

    fn to_int<T: num::PrimInt>(&self) -> Result<T> {
        use Error::*;

        if T::zero().is_signed() {
            let max: i128 = T::max_value().to_i128().unwrap();
            let min: i128 = T::min_value().to_i128().unwrap();
            let value: i128 = self.to_i128().unwrap();
            if value > max {
                return Err(ConversionOverflow);
            }
            if value < min {
                return Err(ConversionUnderflow);
            }
            let value: T = T::from(value).ok_or(UnsupportedConversion)?;
            return Ok(value)
        }
        let max: u128 = T::max_value().to_u128().unwrap();
        let min: u128 = T::min_value().to_u128().unwrap();
        let value: u128 = self.to_u128().unwrap();
        if value > max {
            return Err(ConversionOverflow);
        }
        if value < min {
            return Err(ConversionUnderflow);
        }
        let value: T = T::from(value).ok_or(UnsupportedConversion)?;
        Ok(value)
    }
}

impl<T: int::Int> IntIntrospection for T {}