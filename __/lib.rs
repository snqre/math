#![deny(warnings)]

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::Ordering;
use thiserror::Error;
use num_traits::int::PrimInt;

pub static MIN_PRECISION: u8 = 1u8;
pub static MAX_PRECISION: u8 = 38u8;

pub type Decimals = u8;

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

pub fn k<const A: u8, B: PrimInt>(v: B) -> K::<A, B> {
    K::new(v)
}

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn new(v: B) -> Self {
        Self {
            _v: v,
        }
    }
}

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn is_unsigned(&self) -> bool {

    }
}

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn cast<const C: u8>(&self) -> Result<K<C, B>> {
        let old_decimals: u32 = _only_safe_precision(A)?.into();
        let new_decimals: u32 = _only_safe_precision(C)?.into();
        let x: &Self = &self;
        let v: &B = &x._v;
        let is_match: bool = old_decimals == new_decimals;
        if !is_match {

        }
    }
}

impl<const A: u8, B: PrimInt> Add for K<A, B> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        use Error::*;
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_add(v_1).ok_or(Overflow)?;
        Ok(k(v_2))
    }
}

impl<const A: u8, B: PrimInt> Mul for K<A, B> {
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let decimals: u32 = _only_safe_precision(A)?.into();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        
    }
}

impl<const A: u8, B: PrimInt> Div for K<A, B> {
    fn div(self, rhs: Self) -> Self::Output {
        let decimals: u32 = _only_safe_precision(A)?.into();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        if x.is_unsigned() {
            debug_assert!(y.is_unsigned());
            let v_0: u128 = v_0.to_u128().unwrap();
            let v_1: u128 = v_1.to_u128().unwrap();
            let scale: u128 = 10u128.pow(decimals);
            if !scale.is_power_of_two() {
                let v_2: u128 = v_0
                    .checked_mul(scale)
                    .ok_or(Error::Overflow)?
                    .checked_div(v_1)
                    .ok_or(Error::DivisionByZero)?;
                let max: u128 = B::max_value().to_u128().unwrap();
                let v_2_le_max: bool = v_2 <= max;
                if !v_2_le_max {
                    Err(Error::Overflow)
                } else {
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            } else {
                let scale_shift: u32 = scale.trailing_zeros();
                let v_2: u128 = v_0
                    .checked_shl(scale_shift)
                    .unwrap()
                    .checked_div(v_1)
                    .ok_or(Error::DivisionByZero)?;

            }
        }
    }
}

impl<const A: u8, B: PrimInt> Sub for K<A, B> {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        use Error::*;
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_sub(v_1).ok_or(Underflow)?;
        Ok(k(v_2))
    }
}

impl<const A: u8, B: PrimInt> PartialOrd for K<A, B> {
    fn gt(&self, other: &Self) -> bool {
        self._v > other._v
    }

    fn lt(&self, other: &Self) -> bool {
        self._v < other._v
    }

    fn ge(&self, other: &Self) -> bool {
        self._v >= other._v
    }

    fn le(&self, other: &Self) -> bool {
        self._v <= other._v
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const A: u8, B: PrimInt> Ord for K<A, B> {
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else if self < other {
            other
        } else {
            self
        }
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else if self > other {
            other
        } else {
            self
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self > other {
            Ordering::Greater
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl<const A: u8, B: PrimInt> PartialEq for K<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self._v == other._v
    }
}

impl<const A: u8, B: PrimInt> Eq for K<A, B> {}

fn _only_safe_precision(decimals: Decimals) -> Result<Decimals> {
    if decimals > MAX_PRECISION {
        Err(Error::PrecisionTooLarge)
    } else if decimals < MIN_PRECISION {
        Err(Error::PrecisionTooSmall)
    } else {
        Ok(decimals)
    }
}

mod extension;
mod trait_;
mod player;