use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::fmt::Debug;
use std::fmt::Display;
use std::cmp::Ordering;
use thiserror::Error;

pub trait Q128I<E>:
    Sized
    + Debug
    + Display
    + Clone
    + Copy
    + Add<Output = Result<Self, E>>
    + Sub<Output = Result<Self, E>>
    + Mul<Output = Result<Self, E>>
    + Div<Output = Result<Self, E>>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord {
    /// ***Brief***
    /// Will cast both `F128I` to the largest precision and perform the operation,
    /// the result will be of the largest precision.
    fn c_add(self, rhs: Self) -> Result<Self, E>;
    fn c_sub(self, rhs: Self) -> Result<Self, E>;
    fn c_mul(self, rhs: Self) -> Result<Self, E>;
    fn c_div(self, rhs: Self) -> Result<Self, E>;
    fn cast(self, new_precision: u8) -> Result<Self, E>;
    fn sqrt(self) -> Result<Self, E>;
    fn pow(self, exponent: Self) -> Result<Self, E>;
    fn cos(self) -> Result<Self, E>;
    fn sin(self) -> Result<Self, E>;
    fn tan(self) -> Result<Self, E>;
}


// ---

#[macro_export]
macro_rules! q128 {
    ($value:expr, $precision:expr) => {
        Q128::new($value, $precision)
    };
}

pub static Q_128_MIN_PRECISION: u8 = 1u8;
pub static Q_128_MAX_PRECISION: u8 = 38u8;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Q128EPrecisionFailurePayload {
    pub actual: u8,
    pub min: u8,
    pub max: u8,
}

pub type Q128R<T> = Result<T, Q128E>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum Q128E {
    #[error("Q128: Result is too large to be represented by a `u128`.")]
    Overflow,
    #[error("Q128: Result is too small to be represented by a `u128`.")]
    Underflow,
    #[error("Q128: Division by zero.")]
    DivisionByZero,
    #[error("Q128: Precision of the operands are incompatible.")]
    IncompatiblePrecision(u8, u8),
    #[error("Q128: Precision too small.")]
    PrecisionTooSmall(Q128EPrecisionFailurePayload),
    #[error("Q128: Precision too large.")]
    PrecisionTooLarge(Q128EPrecisionFailurePayload),
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Q128 {
    value: u128,
    precision: u8,
}

impl Q128 {
    pub fn new(value: u128, precision: u8) -> Q128R<Self> {
        only_safe_precision(precision)?;
        let result: Self = Self {
            value,
            precision
        };
        Ok(result)
    }

    pub fn new_zero() -> Self {
        Self::new(0u128, 2u8).expect("")
    }
}

impl Q128I<Q128E> for Q128 {
    fn c_add(self, rhs: Self) -> Q128R<Self> {
        let (x, y, _) = cast_to_highest_precision(self, rhs)?;
        Self::add(x, y)
    }

    fn c_sub(self, rhs: Self) -> Q128R<Self> {
        let (x, y, _) = cast_to_highest_precision(self, rhs)?;
        Self::sub(x, y)
    }

    fn c_mul(self, rhs: Self) -> Q128R<Self> {
        let (x, y, _) = cast_to_highest_precision(self, rhs)?;
        Self::mul(x, y)
    }

    fn c_div(self, rhs: Self) -> Q128R<Self> {
        let (x, y, _) = cast_to_highest_precision(self, rhs)?;
        Self::div(x, y)
    }

    fn cast(self, new_precision: u8) -> Q128R<Self> {
        only_safe_precision(self.precision)?;
        only_safe_precision(new_precision)?;
        if self.precision == new_precision {
            return Ok(self)
        }
        let old_precision: u8 = self.precision;
        let old_precision: u32 = old_precision.into();
        let new_precision: u8 = self.precision;
        let new_precision: u32 = new_precision.into();
        let old_scale: u128 = 10u128.checked_pow(old_precision).ok_or(Q128E::Overflow).expect("F128: Failed `**` operation. `decimals` exceeded 38 (.00000000000000000000000000000000000000).");
        let new_scale: u128 = 10u128.checked_pow(new_precision).ok_or(Q128E::Overflow).expect("F128: Failed `**` operation. `decimals` exceeded 38 (.00000000000000000000000000000000000000).");
        let result: u128 = self.value.checked_mul(new_scale).ok_or(Q128E::Overflow)?;
        let result: u128 = result.checked_div(old_scale).ok_or(Q128E::DivisionByZero)?;
        let precision: u8 = new_precision.try_into().expect("F128: Failed to cast `u32` to `u8`.");
        let result: Self = Self::new(result, precision)?;
        Ok(result)
    }

    fn sqrt(self) -> Q128R<Self> {
        let zero: Self = Self::new_zero();
        if self == zero {
            return Ok(zero)
        }
        let a: Self = Self::new(10u128, 1u8).expect("").cast(self.precision)?;
        let b: Self = Self::new(20u128, 1u8).expect("").cast(self.precision)?;
        let mut x: Self = self;
        let mut y: Self = (((x + a)?) / b)?;
        while y < x {
            x = y;
            y = (((x + self)? / x)? / b)?;
        }
        Ok(x)
    }

    fn pow(self, exponent: Self) -> Q128R<Self> {
        only_safe_precision(self.precision)?;
        only_safe_precision(exponent.precision)?;
        only_compatible_precision(self.precision, exponent.precision)?;
        let precision: u32 = self.precision.into();
        let mut base: u128 = self.value;
        let mut exponent: u128 = exponent.value;
        let mut result: u128 = 1u128;
        let scale: u128 = 10u128.checked_pow(precision).expect("");
        while exponent > 0u128 {
            if exponent % 2u128 == 1u128 {
                result = result.checked_mul(base).ok_or(Q128E::Overflow)?;
                result = result.checked_div(scale).ok_or(Q128E::DivisionByZero)?;
            }
            base = base.checked_mul(base).ok_or(Q128E::Overflow)?;
            base = base.checked_div(scale).ok_or(Q128E::Overflow)?;
            exponent = exponent.checked_div(2u128).expect("");
        }
        Q128::new(result, self.precision)
    }

    fn cos(self) -> Q128R<Self> {
        only_safe_precision(self.precision)?;
        


        let x: Self = self;
        let x_0: Self = (x * x)?;
        let x_1: Self = (x_0 * x_0)?;
        let x_2: Self = (x_1 * x_0)?;
        let a: Self = Q128::new(20u128, 1u8)?;
        let a: Self = a.cast(self.precision)?;
        let b: Self = Q128::new(240u128, 1u8)?;
        let b: Self = b.cast(self.precision)?;
        let c: Self = Q128::new(7200u128, 1u8)?;
        let c: Self = c.cast(self.precision)?;
        let term_0: Self = Q128::new(10u128, 1u8)?;
        let term_0: Self = term_0.cast(self.precision)?;
        let term_1: Self = (x_0 / a)?;
        let term_2: Self = (x_1 / b)?;
        let term_3: Self = (x_2 / c)?;
        let result: Self = (((term_0 - term_1)? + term_2)? - term_3)?;
        Ok(result)
    }

    fn sin(self) -> Q128R<Self> {
        only_safe_precision(self.precision)?;
        let x: Self = self;
        let x_0: Self = (x * x)?;
        let x_1: Self = (x_0 * x)?;
        let x_2: Self = (x_1 * x)?;
        let x_3: Self = (x_2 * x_0)?;
        let x_4: Self = (x_3 * x_0)?;
        let a: Self = Q128::new(60u128, 1u8)?;
        let a: Self = a.cast(self.precision)?;
        let b: Self = Q128::new(1200u128, 1u8)?;
        let b: Self = b.cast(self.precision)?;
        let c: Self = Q128::new(5040u128, 1u8)?;
        let c: Self = c.cast(self.precision)?;
        let term_0: Self = x;
        let term_1: Self = (x_1 / a)?;
        let term_2: Self = (x_2 / b)?;
        let term_3: Self = (x_4 / c)?;
        let result: Self = (((term_0 - term_1)? + term_2)? - term_3)?;
        Ok(result)
    }

    fn tan(self) -> Q128R<Self> {
        let sin: Self = self.sin()?;
        let cos: Self = self.cos()?;
        let result: Self = (sin / cos)?;
        Ok(result)
    }
}

impl Debug for Q128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.value, self.precision)
    }
}

impl Display for Q128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.value, self.precision)
    }
}

impl Add for Q128 {
    type Output = Q128R<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        only_safe_precision(self.precision)?;
        only_safe_precision(rhs.precision)?;
        only_compatible_precision(self.precision, rhs.precision)?;
        let result: u128 = self.value.checked_add(rhs.value).ok_or(Q128E::Overflow)?;
        let result: Self = Q128 {
            value: result,
            precision: self.precision,
        };
        Ok(result)
    }
}

impl Sub for Q128 {
    type Output = Q128R<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        only_safe_precision(self.precision)?;
        only_safe_precision(rhs.precision)?;
        only_compatible_precision(self.precision, rhs.precision)?;
        let result: u128 = self.value.checked_sub(rhs.value).ok_or(Q128E::Underflow)?;
        let result: Self = Self {
            value: result,
            precision: self.precision,
        };
        Ok(result)
    }
}

impl Mul for Q128 {
    type Output = Q128R<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        only_safe_precision(self.precision)?; 
        only_safe_precision(rhs.precision)?;
        only_compatible_precision(self.precision, rhs.precision)?;

        let decimals: u32 = self.precision.into();

        // Calculate scale based on precision
        let scale: u128 = 10u128.checked_pow(decimals).ok_or(Q128E::Overflow)?;

        // Perform the multiplication and scale
        let result: u128 = self.value.checked_mul(rhs.value).ok_or(Q128E::Overflow)?;
        let scaled_result: u128 = result.checked_div(scale).ok_or(Q128E::Overflow)?;

        let result: Self = Self {
            value: scaled_result,
            precision: self.precision,
        };
        
        Ok(result)
    }
}

impl Div for Q128 {
    type Output = Q128R<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        only_safe_precision(self.precision)?;
        only_safe_precision(rhs.precision)?;
        only_compatible_precision(self.precision, rhs.precision)?;
        let decimals: u32 = self.precision.into();
        let scale: u128 = 10u128.checked_pow(decimals).expect("");
        if !scale.is_power_of_two() {
            let result: u128 = self.value.checked_mul(scale).ok_or(Q128E::Overflow)?;
            let result: u128 = result.checked_div(rhs.value).ok_or(Q128E::DivisionByZero)?;
            let result: Self = Self {
                value: result,
                precision: self.precision,
            };
            return Ok(result)
        }
        let scale_shift: u32 = scale.trailing_zeros();
        let result: u128 = self.value.checked_shl(scale_shift).expect("..");
        let result: u128 = result.checked_div(rhs.value).ok_or(Q128E::DivisionByZero)?;
        let result: Self = Self {
            value: result,
            precision: self.precision,
        };
        Ok(result)
    }
}

impl PartialEq for Q128 {
    fn eq(&self, other: &Self) -> bool {
        let largest_decimals: u8 = self.precision.max(other.precision);
        if let (Ok(x), Ok(y)) = (self.cast(largest_decimals), other.cast(largest_decimals)) {
            return x.value == y.value
        }
        false
    }
}

impl Eq for Q128 {}

impl PartialOrd for Q128 {
    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }

    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Q128 {
    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized, {
        if self < min {
            return min
        }
        if self > max {
            return max
        }
        self
    }

    fn max(self, other: Self) -> Self where Self: Sized, {
        if self > other {
            return self
        }
        if self < other {
            return other
        }
        self
    }

    fn min(self, other: Self) -> Self where Self: Sized, {
        if self < other {
            return self
        }
        if self > other {
            return self
        }
        self
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self > other {
            return Ordering::Greater
        }
        if self < other {
            return Ordering::Less
        }
        Ordering::Equal
    }
}

fn only_compatible_precision(decimals_0: u8, decimals_1: u8) -> Q128R<()> {
    if decimals_0 != decimals_1 {
        let e: Q128E = Q128E::IncompatiblePrecision(decimals_0, decimals_1);
        return Err(e)
    }
    Ok(())
}

fn only_safe_precision(decimals: u8) -> Q128R<()> {
    if decimals < Q_128_MIN_PRECISION {
        let payload: Q128EPrecisionFailurePayload = Q128EPrecisionFailurePayload {
            actual: decimals,
            min: Q_128_MIN_PRECISION,
            max: Q_128_MAX_PRECISION,
        };
        let e: Q128E = Q128E::PrecisionTooSmall(payload);
        return Err(e)
    }
    if decimals > Q_128_MAX_PRECISION {
        let payload: Q128EPrecisionFailurePayload = Q128EPrecisionFailurePayload {
            actual: decimals,
            min: Q_128_MIN_PRECISION,
            max: Q_128_MAX_PRECISION,
        };
        let e: Q128E = Q128E::PrecisionTooLarge(payload);
        return Err(e)
    }
    Ok(())
}

fn cast_to_highest_precision(x: Q128, y: Q128) -> Q128R<(Q128, Q128, u8)> {
    let largest_decimals: u8 = x.precision.max(y.precision);
    let x: Q128 = x.cast(largest_decimals)?;
    let y: Q128 = y.cast(largest_decimals)?;
    let result: (Q128, Q128, u8) = (x, y, largest_decimals);
    Ok(result)
}


// --

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let x: Q128 = q128!(100u128, 2u8).unwrap();
        let y: Q128 = q128!(100u128, 2u8).unwrap();
        let result: Q128 = (x + y).unwrap();
        let result_ok: Q128 = q128!(200u128, 2u8).unwrap();
        assert_eq!(result, result_ok);
    }
    
    #[test]
    fn sub() {
        let x: Q128 = q128!(150u128, 2u8).unwrap();
        let y: Q128 = q128!(100u128, 2u8).unwrap();
        let result: Q128 = (x - y).unwrap();
        let result_ok: Q128 = q128!(50u128, 2u8).unwrap();
        assert_eq!(result, result_ok);
    }

    #[test]
    fn mul() {
        let x: Q128 = q128!(500u128, 2u8).unwrap();
        let y: Q128 = q128!(50u128, 2u8).unwrap();
        let result: Q128 = (x * y).unwrap();
        let result_ok: Q128 = q128!(250u128, 2u8).unwrap();
        assert_eq!(result, result_ok);
    }

    #[test]
    fn div() {
        let x: Q128 = q128!(500u128, 2u8).unwrap();
        let y: Q128 = q128!(50u128, 2u8).unwrap();
        let result: Q128 = (x / y).unwrap();
        let result_ok: Q128 = q128!(1000u128, 2u8).unwrap();
        assert_eq!(result, result_ok);
    }

    #[test]
    fn pow() {
        let base: Q128 = q128!(500u128, 2u8).unwrap();
        let expo: Q128 = q128!(200u128, 2u8).unwrap();
        let result: Q128 = base
            .pow(expo)
            .unwrap();
        let result_ok: Q128 = q128!(2500u128, 2u8).unwrap();
        assert_eq!(result, result_ok);
    }

    #[test]
    fn cast() {
        let origin: Q128 = q128!(555u128, 2u8).unwrap();
        let result: Q128 = origin
            .cast(18u8)
            .unwrap()
            .cast(2u8)
            .unwrap();
        assert_eq!(result, origin);
    }

    #[test]
    fn cos() {
        let x: Q128 = q128!(50u128, 2u8).unwrap().cos().unwrap();
        assert_eq!(x.value, 99u128);
    }

    #[test]
    fn _s() {
        
    }
}