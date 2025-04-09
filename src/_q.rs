use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::fmt::Debug;
use std::fmt::Display;
use std::cmp::Ordering;
use thiserror::Error;


// --

pub static Q_MIN_PRECISION: u8 = 1u8;
pub static Q_MAX_PRECISION: u8 = 38u8;

pub type QR<T> = Result<T, QE>;

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


// --

pub trait QU128I<E>:
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
    fn value(&self) -> u128;
    fn precision(&self) -> u8;
    fn to_precision(&self, new_precision: u8) -> Result<Self, E>;
    fn sqrt(&self) -> Result<Self, E>;
}


// --

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct QU128 {
    value: u128,
    precision: u8,
}

impl QU128 {
    pub fn new(value: u128, precision: u8) -> QR<Self> {
        only_safe_precision(precision)?;
        let result: Self = Self {
            value,
            precision,
        };
        Ok(result)
    }

    pub fn new_zero(precision: u8) -> QR<Self> {
        QU128::new(0u128, precision)
    }
}

impl QU128I<QE> for QU128 {
    fn value(&self) -> u128 {
        self.value
    }

    fn precision(&self) -> u8 {
        self.precision
    }

    fn to_precision(&self, new_precision: u8) -> Result<Self, QE> {
        
    }

    fn sqrt(&self) -> Result<Self, QE> {
        
    }
}

impl Add for QU128 {
    type Output = QR<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let v_0: u128 = self.value();
        let v_1: u128 = rhs.value();
        let precision_0: u8 = self.precision();
        let precision_1: u8 = rhs.precision();
        only_compatible_precision(precision_0, precision_1)?;
        only_safe_precision(precision_0)?;
        only_safe_precision(precision_1)?;
        let result: u128 = v_0
            .checked_add(v_1)
            .ok_or(QE::Overflow)?;
        let result: Self = Self::new(result, precision_0)?;
        Ok(result)
    }
}

impl Sub for QU128 {
    type Output = QR<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let v_0: u128 = self.value;
        let v_1: u128 = rhs.value;
        let precision_0: u8 = self.precision;
        let precision_1: u8 = rhs.precision;
        only_compatible_precision(precision_0, precision_1)?;
        only_safe_precision(precision_0)?;
        only_safe_precision(precision_1)?;
        let result: Option<u128> = v_0.checked_sub(v_1);
        let result: u128 = result.ok_or(QE::Overflow)?;
        let result: Self = Self::new(result, precision_0)?;
        Ok(result)
    }
}

// --

pub trait QI128I<E>:
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
    fn value(&self) -> i128;
    fn precision(&self) -> u8;
    fn to_precision(&self, new_precision: u8) -> Result<Self, E>;
    fn to_f32(self) -> Result<f32, E>;
    fn sqrt(self) -> Result<Self, E>;
}


// --

#[derive(Clone)]
#[derive(Copy)]
pub struct QI128 {
    value: i128,
    precision: u8,
}

impl QI128 {
    pub fn new(value: i128, precision: u8) -> QR<Self> {
        only_safe_precision(precision)?;
        let result: Self = Self {
            value,
            precision
        };
        Ok(result)
    }

    pub fn new_zero() -> Self {
        Self::new(0i128, 2u8).unwrap()
    }
}

impl QI128I<QE> for QI128 {
    fn value(&self) -> i128 {
        self.value
    }

    fn precision(&self) -> u8 {
        self.precision
    }

    fn to_precision(&self, new_precision: u8) -> QR<Self> {
        let old_precision: u8 = self.precision;
        QUtil::only_safe_precision(old_precision)?;
        QUtil::only_safe_precision(new_precision)?;
        if old_precision == new_precision {
            let result: Self = self.to_owned();
            return Ok(result)
        }
        let old_scale: i128 = QUtil::scale_i128(old_precision)?;
        let new_scale: i128 = QUtil::scale_i128(new_precision)?;
        let result: i128 = QUtil::muldiv_i128(self.value, new_scale, old_scale)?;
        let result: Self = Self::new(result, new_precision)?;
        Ok(result)
    }

    fn to_f32(self) -> QR<f32> {
        only_safe_precision(self.precision)?;
        let scale: f32 = scale_i128(self.precision)? as f32;
        let value: f32 = self.value as f32;
        let result: f32 = value / scale;
        Ok(result)
    }

    fn sqrt(self) -> QR<Self> {
        let zero: Self = Self::new_zero();
        if self == zero {
            return Ok(zero)
        }
        let a: Self = Self::new(10i128, 1u8).unwrap().to_precision(self.precision).unwrap();
        let b: Self = Self::new(10i128, 1u8).unwrap().to_precision(self.precision).unwrap();
        let mut x: Self = self;
        let mut y: Self = (((x + a)?) / b)?;
        while y < x {
            x = y;
            y = (((x + self)? / x)? / b)?;
        }
        Ok(x)
    }
}

impl Debug for QI128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let representation: QR<f32> = self.to_f32();
        if let Ok(representation) = representation {
            let representation = representation.to_string();
            write!(f, "{}", representation)
        } else {
            write!(f, "({}:{})", self.value, self.precision)
        }
    }
}

impl Display for QI128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let representation: QR<f32> = self.to_f32();
        if let Ok(representation) = representation {
            let representation = representation.to_string();
            write!(f, "{}", representation)
        } else {
            write!(f, "({}:{})", self.value, self.precision)
        }
    }
}

impl Add for QI128 {
    type Output = QR<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        only_safe_precision(self.precision)?;
        only_safe_precision(rhs.precision)?;
        only_compatible_precision(self.precision, rhs.precision)?;
        let result: i128 = self.value
            .checked_add(rhs.value)
            .ok_or(QE::Overflow)?;
        let result: Self = Self {
            value: result,
            precision: self.precision,
        };
        Ok(result)
    }
}

impl Sub for QI128 {
    type Output = QR<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let v_0: i128 = self.value;
        let v_1: i128 = rhs.value;
        let precision_0: u8 = self.precision;
        let precision_1: u8 = rhs.precision;
        only_compatible_precision(precision_0, precision_1)?;
        only_safe_precision(precision_0)?;
        only_safe_precision(precision_1)?;
        let result: i128 = v_0
            .checked_sub(v_1)
            .ok_or(QE::Underflow)?;
        let result: Self = Self {
            value: result,
            precision: self.precision,
        };
        Ok(result)
    }
}

impl Mul for QI128 {
    type Output = QR<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let v_0: i128 = self.value;
        let v_1: i128 = rhs.value;
        let precision_0: u8 = self.precision;
        let precision_1: u8 = rhs.precision;
        only_compatible_precision(precision_0, precision_1)?;
        only_safe_precision(precision_0)?;
        only_safe_precision(precision_1)?;
        let scale: i128 = scale_i128(precision_0)?;
        let result: i128 = muldiv_i128(v_0, v_1, scale)?;
        let result: Self = Self {
            value: result,
            precision: precision_0,
        };
        Ok(result)
    }
}

impl Div for QI128 {
    type Output = QR<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let v_0: i128 = self.value;
        let v_1: i128 = rhs.value;
        let precision_0: u8 = self.precision;
        let precision_1: u8 = rhs.precision;
        only_compatible_precision(precision_0, precision_1)?;
        only_safe_precision(precision_0)?;
        only_safe_precision(precision_1)?;
        let scale: u128 = scale_u128(precision_0)?;
        if !scale.is_power_of_two() {
            let scale: i128 = scale
                .try_into()
                .unwrap();
            let result: i128 = muldiv_i128(v_0, scale, v_1)?;
            let result: Self = Self {
                value: result,
                precision: precision_0,
            };
            return Ok(result)
        }
        let scale_shift: u32 = scale.trailing_zeros();
        let result: i128 = v_0
            .checked_shl(scale_shift)
            .unwrap()
            .checked_div(v_1)
            .ok_or(QE::DivisionByZero)?;
        let result: Self = Self {
            value: result,
            precision: precision_0,
        };
        Ok(result)
    }
}

impl PartialEq for QI128 {
    fn eq(&self, rhs: &Self) -> bool {
        if let Ok((x, y)) = to_highest_common_precision_qi128::<Self>(self, rhs) {
            return x.value == y.value
        }
        false
    }
}

impl Eq for QI128 {}

impl PartialOrd for QI128 {
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
        if self > other {
            let order: Ordering = Ordering::Greater;
            return Some(order)
        }
        if self < other {
            let order: Ordering = Ordering::Less;
            return Some(order)
        }
        let order: Ordering = Ordering::Equal;
        Some(order)
    }
}

impl Ord for QI128 {
    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized, {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn max(self, other: Self) -> Self where Self: Sized, {
        if self >= other {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self where Self: Sized, {
        if self <= other {
            self
        } else {
            other
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// --


trait QUtilI<T0> {
    fn only_compatible_precision(precision_0: u8, precision_1: u8) -> Result<(), T0>;
    fn only_safe_precision(precision: u8) -> Result<(), T0>;
    fn scale_u128(precision: u8) -> Result<u128, T0>;
    fn scale_i128(precision: u8) -> Result<i128, T0>;
    fn muldiv_u128(x: u128, y: u128, z: u128) -> Result<u128, T0>;
    fn muldiv_i128(x: i128, y: i128, z: i128) -> Result<i128, T0>;
    fn to_highest_common_precision_qu128<T1: QU128I<T0>>(lhs: T1, rhs: T1) -> Result<(T1, T1), T0>;
    fn to_highest_common_precision_qi128<T1: QI128I<T0>>(lhs: T1, rhs: T1) -> Result<(T1, T1), T0>;
}

struct QUtil;

impl QUtilI<QE> for QUtil {
    
    fn only_compatible_precision(precision_0: u8, precision_1: u8) -> QR<()> {
        if precision_0 != precision_1 {
            let e: QE = QE::IncompatiblePrecision(precision_0, precision_1);
            return Err(e)
        }
        Ok(())
    }

    fn only_safe_precision(precision: u8) -> QR<()> {
        if precision < Q_MIN_PRECISION {
            let e: QE = QE::PrecisionTooSmall(precision, Q_MIN_PRECISION, Q_MAX_PRECISION);
            return Err(e)
        }
        if precision > Q_MAX_PRECISION {
            let e: QE = QE::PrecisionTooLarge(precision, Q_MIN_PRECISION, Q_MAX_PRECISION);
            return Err(e)
        }
        Ok(())
    }

    fn scale_u128(precision: u8) -> QR<u128> {
        Self::only_safe_precision(precision)?;
        let precision: u32 = precision.into();
        let result: u128 = 10u128
            .checked_pow(precision)
            .ok_or(QE::Overflow)
            .unwrap();
        Ok(result)
    }

    fn scale_i128(precision: u8) -> QR<i128> {
        let result: i128 = Self::scale_u128(precision)? as i128;
        Ok(result)
    }

    fn muldiv_u128(x: u128, y: u128, z: u128) -> QR<u128> {
        let result: u128 = x
            .checked_mul(y)
            .ok_or(QE::Overflow)?
            .checked_div(z)
            .ok_or(QE::DivisionByZero)?;
        Ok(result)
    }
}


fn only_compatible_precision(precision_0: u8, precision_1: u8) -> QR<()> {
    if precision_0 != precision_1 {
        let e: QE = QE::IncompatiblePrecision(precision_0, precision_1);
        return Err(e)
    }
    Ok(())
}

fn only_safe_precision(precision: u8) -> QR<()> {
    if precision < Q_MIN_PRECISION {
        let e: QE = QE::PrecisionTooSmall(precision, Q_MIN_PRECISION, Q_MAX_PRECISION);
        return Err(e)
    }
    if precision > Q_MAX_PRECISION {
        let e: QE = QE::PrecisionTooLarge(precision, Q_MIN_PRECISION, Q_MAX_PRECISION);
        return Err(e)
    }
    Ok(())
}

fn scale_u128(precision: u8) -> QR<u128> {
    only_safe_precision(precision)?;
    let precision: u32 = precision.into();
    let result: u128 = 10u128
        .checked_pow(precision)
        .ok_or(QE::Overflow)
        .unwrap();
    Ok(result)
}

fn scale_i128(precision: u8) -> QR<i128> {
    only_safe_precision(precision)?;
    let precision: u32 = precision.into();
    let result: i128 = 10i128
        .checked_pow(precision)
        .ok_or(QE::Overflow)
        .unwrap();
    Ok(result)
}

fn muldiv_i128(x: i128, y: i128, z: i128) -> QR<i128> {
    let result: i128 = x
        .checked_mul(y)
        .ok_or(QE::Overflow)?
        .checked_div(z)
        .ok_or(QE::DivisionByZero)?;
    Ok(result)
}


fn to_highest_common_precision_qi128<T: QI128I<QE>>(lhs: &T, rhs: &T) -> QR<(T, T)> {
    let precision_0: u8 = lhs.precision();
    let precision_1: u8 = rhs.precision();
    let highest_common_precision: u8 = precision_0.max(precision_1);
    let lhs: T = lhs.to_precision(highest_common_precision)?;
    let rhs: T = rhs.to_precision(highest_common_precision)?;
    Ok((lhs, rhs))
}