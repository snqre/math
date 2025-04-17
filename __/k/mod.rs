mod extension; pub use extension::*;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::Ordering;
use thiserror::Error;
use num_traits::int::PrimInt;

pub static MIN_PRECISION: u8 = 1u8;
pub static MAX_PRECISION: u8 = 38u8;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum Error {
    #[error("")]
    Overflow,
    #[error("")]
    Underflow,
    #[error("")]
    DivisionByZero,
    #[error("")]
    PrecisionTooSmall,
    #[error("")]
    PrecisionTooLarge,
    #[error("")]
    IncompatiblePrecision,
}

pub struct K<const A: u8, B: PrimInt> {
    _v: B,
}