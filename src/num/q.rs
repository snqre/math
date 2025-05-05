use crate::num::int::{self, Int as _};
use crate::num::int_introspection::{self, IntIntrospection as _};
use crate::num::precision;
use crate::num::engine::default_engine;
use core::ops;

pub trait Engine 
where
    Self: Sized,
    Self: Clone {

    fn sqrt<const A: u8, B>(&self, x: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where 
        B: int::Int,
        B: int_introspection::IntIntrospection,
        precision::Precision<A>: precision::Compatible {
        use Error::*;
        debug_assert!(A >= 1u8);
        debug_assert!(A <= 38u8);
        let scale: u128 = precision::u128_scale::<A>();
        let x: B = x._v;
        if x.is_signed() {
            let x: i128 = x.to_i128().ok_or(UnsupportedConversion)?;
            if x < 0 {
                return Err(NegativeSquareRoot)
            }
            let x: u128 = x.to_u128().ok_or(UnsupportedConversion)?;
            let x: u128 = x * scale;
            let x: u128 = _sqrt(x);
            let engine: Self = self.clone();
            let result: B = B::from(x).ok_or(UnsupportedConversion)?;
            let result: Q<A, B, Self> = new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let x: u128 = x.to_u128().ok_or(UnsupportedConversion)?;
        let x: u128 = x * scale;
        let x: u128 = _sqrt(x);
        let engine: Self = self.clone();
        let result: B = B::from(x).ok_or(UnsupportedConversion)?;
        let result: Q<A, B, Self> = new_with_custom_engine(&result, self);
        Ok(result)
    }

    fn cast<const A: u8, const B: u8, C>(&self, x: &Q<A, C, Self>) -> Result<Q<B, C, Self>> 
    where
        C: int::Int,
        C: int_introspection::IntIntrospection,
        precision::Precision<A>: precision::Compatible,
        precision::Precision<B>: precision::Compatible {
        debug_assert!(A >= 1u8);
        debug_assert!(A <= 38u8);
        let x: C = x._v;
        if A == B {
            let result: C = x;
            let result: Q<B, C, Self> = new_with_custom_engine(&result, self);
            return Ok(result)
        }
        if x.is_signed() {
            let old_scale: i128 = precision::i128_scale::<A>();
            let new_scale: i128 = precision::i128_scale::<B>();
            let result: i128 = x.to_i128().unwrap();
            let result: i128 = self.muldiv(&result, &new_scale, &old_scale)?;
            let result: C = C::from(result).unwrap();
            let result: Q<B, C, Self> = Q::new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let old_scale: u128 = precision::u128_scale::<A>();
        let new_scale: u128 = precision::u128_scale::<B>();
        let result: u128 = x.to_u128().unwrap();
        let result: u128 = self.muldiv(&result, &new_scale, &old_scale)?;
        let result: C = C::from(result).unwrap();
        let result: Q<B, C, Self> = Q::new_with_custom_engine(&result, self);
        Ok(result)
    }

    fn mul<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where     
        B: int::Int,
        B: int_introspection::IntIntrospection,
        precision::Precision<A>: precision::Compatible {
        debug_assert!(A >= 1u8);
        debug_assert!(A <= 38u8);
        let x: B = x._v;
        let y: B = y._v;
        let precision: u32 = A.into();
        if x.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            let scale: i128 = precision::i128_scale::<A>();
            let result: i128 = self.muldiv(&x, &y, &scale)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let scale: u128 = precision::u128_scale::<A>();
        let result: u128 = self.muldiv(&x, &y, &scale)?;
        let result: B = B::from(result).unwrap();
        let result: Q<A, B, Self> = Q::new_with_custom_engine(&result, self);
        Ok(result)
    }

    fn div<const A: u8, B>(
        &self, 
        x: &Q<A, B, Self>, 
        y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where 
        B: int::Int,
        B: int_introspection::IntIntrospection,
        precision::Precision<A>: precision::Compatible {
        debug_assert!(A >= 1u8);
        debug_assert!(A <= 38u8);
        let x: &B = &x._v;
        let y: &B = &y._v;
        let scale: u128 = precision::Precision::<A>::u128_scale();
        if x.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            if scale.is_power_of_two() {
                let z: u32 = scale.trailing_zeros();
                let result: i128 = (x << z).checked_div(y).ok_or(Error::DivisionByZero)?;
                let result: &B = &B::from(result).unwrap();
                let result: Q<A, B, Self> = Q::new_with_custom_engine(result, self);
                return Ok(result)
            }
            let scale: i128 = precision::i128_scale::<A>();
            let result: i128 = self.muldiv(&x, &scale, &y)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = Q::new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        if scale.is_power_of_two() {
            let z: u32 = scale.trailing_zeros();
            let result: u128 = (x << z).checked_div(y).ok_or(Error::DivisionByZero)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = Q::new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let result: u128 = self.muldiv(&x, &scale, &y)?;
        let result: B = B::from(result).unwrap();
        let result: Q<A, B, Self> = Q::new_with_custom_engine(&result, self);
        Ok(result)
    }

    fn muldiv<'a, T>(
        &self, 
        x: &'a T, 
        y: &'a T, 
        z: &'a T) -> Result<T> 
    where 
        T: int::Int,
        T: int_introspection::IntIntrospection {
        if z == &T::zero() {
            return Err(Error::DivisionByZero);
        }
        if z.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            let z: i128 = z.to_i128().unwrap();
            let sign: i128 = ((x ^y ^ z) >> 127) & 1;
            let x: u128 = x.unsigned_abs();
            let y: u128 = y.unsigned_abs();
            let z: u128 = z.unsigned_abs();
            let (a, b) = _wide_mul(x, y);
            if b >= z {
                return Err(Error::Overflow)
            }
            if b == 0 {
                let result: u128 = a / z;
                if sign == 1 {
                    let result: T = result
                        .to_i128()
                        .ok_or(Error::UnsupportedConversion)?
                        .wrapping_neg()
                        .to_int()
                        .ok_or(Error::UnsupportedConversion)?;
                    return Ok(result)
                }
                let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
                return Ok(result)
            }
            let result: u128 = _fold_128_bit_product_mod(a, b, z) / z;
            if sign == 1 {
                let result: T = result
                    .to_i128()
                    .ok_or(Error::UnsupportedConversion)?
                    .wrapping_neg()
                    .to_int()
                    .ok_or(Error::UnsupportedConversion)?;
                return Ok(result)
            }
            let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let z: u128 = z.to_u128().unwrap();
        let (a, b) = _wide_mul(x, y);
        if b >= z {
            return Err(Error::Overflow);
        }
        if b == 0 {
            let result: u128 = a / z;
            let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
            return Ok(result);
        }
        let result: u128 = _fold_128_bit_product_mod(a, b, z) / z;
        let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }
}

fn _sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0
    }
    let mut x_0 = n / 2;
    let mut x_1 = (x_0 + n / x_0) / 2;
    while x_1 < x_0 {
        x_0 = x_1;
        x_1 = (x_0 + n / x_0) / 2;
    }
    x_0
}

fn _fold_128_bit_product_mod(a: u128, b: u128, z: u128) -> u128 {
    (((((b % z) << 64) | (a >> 64)) % z) << 64) | (a & 0xFFFFFFFFFFFFFFFF)
}

fn _wide_mul(x: u128, y: u128) -> (u128, u128) {
    let x_hi: u128 = x >> 64;
    let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
    let y_hi: u128 = y >> 64;
    let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
    let lo_lo: u128 = x_lo * y_lo;
    let lo_hi: u128 = x_lo * y_hi;
    let hi_lo: u128 = x_hi * y_lo;
    let hi_hi: u128 = x_hi * y_hi;
    let m: u128 = lo_hi + hi_lo;
    let c: u128 = ((m < lo_hi) as u128) << 64;
    let m_lo: u128 = m << 64;
    let m_hi: u128 = m >> 64;
    let a: u128 = lo_lo.wrapping_add(m_lo);
    let b: u128 = hi_hi + m_hi + c + ((a < lo_lo) as u128);
    (a, b)
}

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("This value is above the representable range for this type.")]
    Overflow,
    #[error("This value is above the representable range for the type it is being converted to.")]
    OverflowOnConversion,
    #[error("This value is below the representable range for this type.")]
    Underflow,
    #[error("This value is below the representable range for the type it is being converted to.")]
    UnderflowOnConversion,
    #[error("Cannot divide by zero.")]
    DivisionByZero,
    #[error("This value is above or below the representable range.")]
    UnsupportedConversion,
    #[error("")]
    NegativeSquareRoot
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B, C> 
where 
    B: int::Int, 
    C: Engine, 
    precision::Precision<A>: precision::Compatible {
    _v: B,
    _engine: C
}

pub fn new<const A: u8, B>(value: &B) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    precision::Precision<A>: precision::Compatible {
    Q {
        _v: *value,
        _engine: default_engine::new()
    }
}

pub fn new_with_custom_engine<const A: u8, B, C>(value: &B, engine: &C) -> Q<A, B, C>
where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    C: Engine,
    precision::Precision<A>: precision::Compatible {
    Q { _v: *value, _engine: engine.clone() }
}






impl<const A: u8, B, C> ops::Add for Q<A, B, C> 
    where 
        B: int::Int,
        C: Engine, 
        precision::Precision<A>: precision::Compatible {
 
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(A >= 1u8);
        debug_assert!(A <= 38u8);
        let x: &Self = &self;
        let x: &B = &x._v;
        let y: &Self = &rhs;
        let y: &B = &y._v;
        let result: B = x.checked_add(y).ok_or(Error::Overflow)?;
        Ok(new_with_custom_engine(&result, &self._engine))
    }
}

impl<const A: u8, B, C> ops::Sub for Q<A, B, C> 
where 
    B: int::Int,
    C: Engine, 
    precision::Precision<A>: precision::Compatible {

    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(A >= 1u8);
        debug_assert!(A <= 38u8);
        let x: &Self = &self;
        let x: &B = &x._v;
        let y: &Self = &rhs;
        let y: &B = &y._v;
        let result: B = x.checked_sub(y).ok_or(Error::Overflow)?;
        Ok(new_with_custom_engine(&result, &self._engine))
    }
}