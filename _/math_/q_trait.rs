use crate::math::brandable_t::BrandableT;
use crate::math::sign_introspection_trait::SignIntrospectionTrait;
use core::ops::Add as AddTrait;
use core::ops::Sub as SubTrait;
use core::ops::Mul as MulTrait;
use core::ops::Div as DivTrait;
use core::ops::Rem as RemTrait;
use num_traits::int::PrimInt as PrimIntTrait;
use thiserror::Error;

pub trait QTrait<const A: u8, B>
    where
        Self: Sized,
        Self: BrandableT,
        Self: SignIntrospectionTrait,
        Self: AddTrait<Output = QTraitResult<Self>>,
        Self: SubTrait<Output = QTraitResult<Self>>,
        Self: MulTrait<Output = QTraitResult<Self>>,
        Self: DivTrait<Output = QTraitResult<Self>>,
        Self: RemTrait<Output = QTraitResult<Self>>,
        B: PrimIntTrait,
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