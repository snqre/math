pub mod q_i_128;
pub mod q_u_128;
pub mod q_util;

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use thiserror::Error;
use num_traits::Unsigned;

// A `Q` unit is a fixed point representation. Is less performant in terms of
// calculations than a floating point but more widely available on many low
// level systems, and offers reliable, and precise math.

pub static Q_MIN_PRECISION: u8 = 1u8;
pub static Q_MAX_PRECISION: u8 = 38u8;

pub type QR<T0> = Result<T0, QE>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Error)]
pub enum QE {
    #[error("Q: Result is too large to be represented.")]
    Overflow,
    #[error("Q: Result is too small to be represented.")]
    Underflow,
    #[error("Q: Division by zero.")]
    DivisionByZero,
    #[error("Q: Precision of the operands are incompatible.")]
    IncompatiblePrecision(u8, u8),
    #[error("Q: Precision too small.")]
    PrecisionTooSmall(u8, u8, u8),
    #[error("Q: Precision too large.")]
    PrecisionTooLarge(u8, u8, u8),
}

pub trait Q<T0: Unsigned, T1>:
    Sized
    + Debug
    + Display
    + Clone
    + Copy
    + Add<Output = Result<Self, T1>>
    + Sub<Output = Result<Self, T1>>
    + Mul<Output = Result<Self, T1>>
    + Div<Output = Result<Self, T1>>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord {
    fn value(self) -> T0;
    fn precision(self) -> u8;
    fn sqrt(self) -> Result<Self, T1>;
    fn to_precision(self, new_precision: u8) -> Result<Self, T1>;
}