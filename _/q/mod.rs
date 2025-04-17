use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::Ordering;
use thiserror::Error;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use num_traits::int::PrimInt;

mod add;
mod constructor;
mod display;
mod div;
mod eq;
mod mul;
mod ord;
mod partial_eq;
mod partial_ord;
mod sub;

// Any value larger below 1, and above 38 are unrepresentable across all integer ranges.
pub static Q_MAX_PRECISION: u32 = 38u32;
pub static Q_MIN_PRECISION: u32 = 1u32;

pub type QResult<T> = Result<T, QError>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum QError {
    #[error("Q: Result is too large to be represented.")]
    Overflow,
    #[error("Q: Result is too small to be represented.")]
    Underflow,
    #[error("Q: Division by zero.")]
    DivisionByZero,
    #[error("Q: Precision of the operands are incompatible.")]
    IncompatiblePrecision,
    #[error("Q: Precision too small.")]
    PrecisionTooSmall,
    #[error("")]
    PrecisionTooLarge,
    #[error("")]
    Unrepresentable,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<T: _T> {
    _value: T,
    _precision: u32,
}

impl<T: _T> Q<T> {
    fn _only_compatible_precision(precision_0: u32, precision_1: u32) -> QResult<()> {
        if precision_0 != precision_1 {
            let e: QError = QError::IncompatiblePrecision;
            return Err(e)
        }
        Ok(())
    }
    
    fn _only_non_zero_precision(precision: u32) -> QResult<()> {
        if precision > Q_MAX_PRECISION {
            let e: QError = QError::PrecisionTooLarge;
            return Err(e)
        }
        if precision < Q_MIN_PRECISION {
            let e: QError = QError::PrecisionTooSmall;
            return Err(e)
        }
        Ok(())
    }
}

trait _T: _TUtil + PrimInt + FromPrimitive + ToPrimitive {}

trait _TUtil: Debug + Display {}