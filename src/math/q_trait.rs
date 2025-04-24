use crate::math::branded_trait::BrandedTrait;
use crate::math::sign_introspection_trait::SignIntrospectionTrait;
use core::ops::Add as AddTrait;
use core::ops::Sub as SubTrait;
use core::ops::Mul as MulTrait;
use core::ops::Div as DivTrait;
use core::ops::Rem as RemTrait;
use core::convert::TryFrom as TryFromTrait;
use core::convert::TryInto as TryIntoTrait;
use num_traits::int::PrimInt as PrimIntTrait;
use thiserror::Error;

pub trait QTrait<const A: u8, B: PrimIntTrait>:
      Sized
    + BrandedTrait
    + SignIntrospectionTrait
    + AddTrait<Output = QTraitResult<Self>>
    + SubTrait<Output = QTraitResult<Self>>
    + MulTrait<Output = QTraitResult<Self>>
    + DivTrait<Output = QTraitResult<Self>>
    + RemTrait<Output = QTraitResult<Self>>
    + TryIntoTrait<i8, Error = QTraitError>
    + TryIntoTrait<i16, Error = QTraitError>
    + TryIntoTrait<i32, Error = QTraitError>
    + TryIntoTrait<i64, Error = QTraitError>
    + TryIntoTrait<i128, Error = QTraitError>
    + TryIntoTrait<isize, Error = QTraitError>
    + TryIntoTrait<u8, Error = QTraitError>
    + TryIntoTrait<u16, Error = QTraitError>
    + TryIntoTrait<u32, Error = QTraitError>
    + TryIntoTrait<u64, Error = QTraitError>
    + TryIntoTrait<u128, Error = QTraitError>
    + TryIntoTrait<usize, Error = QTraitError>
    + TryIntoTrait<f32, Error = QTraitError>
    + TryIntoTrait<f64, Error = QTraitError>
    + TryFromTrait<i8, Error = QTraitError>
    + TryFromTrait<i16, Error = QTraitError>
    + TryFromTrait<i32, Error = QTraitError>
    + TryFromTrait<i64, Error = QTraitError>
    + TryFromTrait<i128, Error = QTraitError>
    + TryFromTrait<isize, Error = QTraitError>
    + TryFromTrait<u8, Error = QTraitError>
    + TryFromTrait<u16, Error = QTraitError>
    + TryFromTrait<u32, Error = QTraitError>
    + TryFromTrait<u64, Error = QTraitError>
    + TryFromTrait<u128, Error = QTraitError>
    + TryFromTrait<usize, Error = QTraitError>
    + TryFromTrait<f32, Error = QTraitError>
    + TryFromTrait<f64, Error = QTraitError>
    {}

pub type QTraitResult<T> = Result<T, QTraitError>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum QTraitError {
    #[error("")]
    Overflow,
    #[error("")]
    Underflow,
    #[error("")]
    DivisionByZero,
    #[error("")]
    RemByZero,
    #[error("")]
    PrecisionTooSmall,
    #[error("")]
    PrecisionTooLarge,
    #[error("")]
    IncompatiblePrecision,
    #[error("")]
    ConversionFailure,
}