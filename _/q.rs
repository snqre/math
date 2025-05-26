use crate::int;
use crate::precision;
use crate::default_engine;
use crate::math;
use crate::trig;

use ::core::ops;
use ::core::fmt;
use ::core::cmp;

use int::Introspection as _;
use num_traits::ToPrimitive as _;

// The Engine is expected to be stateless and light as it will be copied around several
// times.
pub trait Engine 
where
    Self: Sized,
    Self: Clone,
    Self: Copy {

    fn tan<const A: usize, B>(&self, value: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Int,
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        todo!()
    }

    fn cos<const A: usize, B>(&self, value: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Int,
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        let result: Q<A, B, Self> = self.sub(&from_int_with_custom_engine(90, *self)?, value)?;
        let result: Q<A, B, Self> = self.sin(&result)?;
        Ok(result)
    }

    // Bhaskara's sine approximation
    fn sin<const A: usize, B>(&self, value: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where
        B: int::Int,
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        let lf: Q<A, B, Self> = self.mul(&from_int_with_custom_engine(4, *self)?, value)?;
        let df: Q<A, B, Self> = self.sub(&from_int_with_custom_engine(180, *self)?, &lf)?;
        let nm: Q<A, B, Self> = self.mul(&lf, &df)?;
        let result: Q<A, B, Self> = self.div(&nm, &from_int_with_custom_engine(405, *self)?)?;
        Ok(result)
    }

    fn sqrt<const A: usize, B>(&self, x: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where 
        B: int::Introspection, 
        precision::Precision<A>: precision::Compatible {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let scale: u128 = precision::SCALE_U128[A];
        let value: B = x._v;
        if value.is_signed() {
            let x: i128 = value.to_i128().ok_or(Error::UnsupportedConversion)?;
            if x < 0 {
                return Err(Error::NegativeSquareRoot)
            }
            let x: u128 = x.try_into().ok().ok_or(Error::UnsupportedConversion)?;
            let x: u128 = x * scale;
            let x: u128 = math::sqrt(x);
            let result: B = B::from(x).ok_or(Error::UnsupportedConversion)?;
            let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
            return Ok(result)
        }
        let x: u128 = value.to_u128().ok_or(Error::UnsupportedConversion)?;
        let x: u128 = x * scale;
        let x: u128 = math::sqrt(x);
        let result: B = B::from(x).ok_or(Error::UnsupportedConversion)?;
        let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
        Ok(result)
    }

    fn cast<const A: usize, const B: usize, C>(&self, x: &Q<A, C, Self>) -> Result<Q<B, C, Self>> 
    where
        C: int::Int,
        C: int::Introspection,
        precision::Precision<A>: precision::Compatible,
        precision::Precision<B>: precision::Compatible {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        debug_assert!(B >= 1);
        debug_assert!(B <= 38);
        let value: C = x._v;
        if A == B {
            let result: C = value;
            let result: Q<B, C, Self> = new_with_custom_engine(result, self.clone());
            return Ok(result)
        }
        if value.is_signed() {
            let old_scale: i128 = precision::SCALE_I128[A];
            let new_scale: i128 = precision::SCALE_I128[B];
            let result: i128 = value.to_i128().unwrap();
            let result: i128 = self.muldiv(&result, &new_scale, &old_scale)?;
            let result: C = C::from(result).unwrap();
            let result: Q<B, C, Self> = new_with_custom_engine(result, self.clone());
            return Ok(result)
        }
        let old_scale: u128 = precision::SCALE_U128[A];
        let new_scale: u128 = precision::SCALE_U128[B];
        let result: u128 = value.to_u128().unwrap();
        let result: u128 = self.muldiv(&result, &new_scale, &old_scale)?;
        let result: C = C::from(result).unwrap();
        let result: Q<B, C, Self> = new_with_custom_engine(result, self.clone());
        Ok(result)
    }

    fn add<const A: usize, B>(
        &self,
        x: &Q<A, B, Self>,
        y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: B = x._v;
        let y: B = y._v;
        let result: B = x.checked_add(&y).ok_or(Error::Overflow)?;
        Ok(new_with_custom_engine(result, *self))
    }

    fn sub<const A: usize, B>(
        &self,
        x: &Q<A, B, Self>,
        y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: B = x._v;
        let y: B = y._v;
        let result: B = x.checked_sub(&y).ok_or(Error::Underflow)?;
        Ok(new_with_custom_engine(result, *self))
    }

    fn mul<const A: usize, B>(
        &self, 
        x: &Q<A, B, Self>, 
        y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> where B: int::Introspection, precision::Precision<A>: precision::Compatible {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: B = x._v;
        let y: B = y._v;
        if x.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            let scale: i128 = precision::SCALE_I128[A];
            let result: i128 = self.muldiv(&x, &y, &scale)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(result, *self);
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let scale: u128 = precision::SCALE_U128[A];
        let result: u128 = self.muldiv(&x, &y, &scale)?;
        let result: B = B::from(result).unwrap();
        let result: Q<A, B, Self> = new_with_custom_engine(result, *self);
        Ok(result)
    }

    fn div<const A: usize, B>(
        &self, 
        x: &Q<A, B, Self>, 
        y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where 
        B: int::Int,
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: &B = &x._v;
        let y: &B = &y._v;
        let scale: u128 = precision::SCALE_U128[A];
        if x.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            if scale.is_power_of_two() {
                let z: u32 = scale.trailing_zeros();
                let result: i128 = (x << z).checked_div(y).ok_or(Error::DivisionByZero)?;
                let result: B = B::from(result).unwrap();
                let result: Q<A, B, Self> = new_with_custom_engine(result, *self);
                return Ok(result)
            }
            let scale: i128 = precision::SCALE_I128[A];
            let result: i128 = self.muldiv(&x, &scale, &y)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(result, *self);
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        if scale.is_power_of_two() {
            let z: u32 = scale.trailing_zeros();
            let result: u128 = (x << z).checked_div(y).ok_or(Error::DivisionByZero)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(result, *self);
            return Ok(result)
        }
        let result: u128 = self.muldiv(&x, &scale, &y)?;
        let result: B = B::from(result).unwrap();
        let result: Q<A, B, Self> = new_with_custom_engine(result, *self);
        Ok(result)
    }

    fn muldiv<'a, T>(
        &self, 
        x: &'a T, 
        y: &'a T, 
        z: &'a T) -> Result<T> 
    where 
        T: int::Int,
        T: int::Introspection {
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
            let (a, b) = math::wide_mul(x, y);
            if b >= z {
                return Err(Error::Overflow)
            }
            if b == 0 {
                let result: u128 = a / z;
                if sign == 1 {
                    let result: i128 = result.try_into().ok().ok_or(Error::UnsupportedConversion)?;
                    let result: T = result.wrapping_neg().to_int().ok_or(Error::UnsupportedConversion)?;
                    return Ok(result)
                }
                let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
                return Ok(result)
            }
            let result: u128 = math::fold_128_bit_product_mod(a, b, z) / z;
            if sign == 1 {
                let result: i128 = result.try_into().ok().ok_or(Error::UnsupportedConversion)?;
                let result: T = result.wrapping_neg().to_int().ok_or(Error::UnsupportedConversion)?;
                return Ok(result)
            }
            let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let z: u128 = z.to_u128().unwrap();
        let (a, b) = math::wide_mul(x, y);
        if b >= z {
            return Err(Error::Overflow);
        }
        if b == 0 {
            let result: u128 = a / z;
            let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
            return Ok(result);
        }
        let result: u128 = math::fold_128_bit_product_mod(a, b, z) / z;
        let result: T = result.to_int().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }
}

macro_rules! _variant_u8 {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision U8>] = [<Q $precision>]<u8>;
            )*
        }
    };
}

_variant_u8!(1, 2);

macro_rules! _variant_u16 {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision U16>] = [<Q $precision>]<u16>;
            )*
        }
    };
}

_variant_u16!(1, 2, 3, 4);

macro_rules! variant_u32_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision U32>] = [<Q $precision>]<u32>;
            )*
        }
    };
}

variant_u32_!(
    1, 2, 3,
    4, 5, 6,
    7, 8, 9
);

macro_rules! variant_u64_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision U64>] = [<Q $precision>]<u64>;
            )*
        }
    };
}

variant_u64_!(
    1, 2, 3,
    4, 5, 6,
    7, 8, 9,
    10, 11, 12,
    13, 14, 15,
    16, 17, 18,
    19
);

macro_rules! variant_u128_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision U128>] = [<Q $precision>]<u128>;
            )*
        }
    };
}

variant_u128_!(
    1, 2, 3,
    4, 5, 6,
    7, 8, 9,
    10, 11, 12,
    13, 14, 15,
    16, 17, 18,
    19, 20, 21,
    22, 23, 24,
    25, 26, 27,
    28, 29, 30,
    31, 32, 33,
    34, 35, 36,
    37, 38
);

macro_rules! variant_i8_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision I8>] = [<Q $precision>]<i8>;
            )*
        }
    };
} 

variant_i8_!(1, 2);

macro_rules! variant_i16_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision I16>] = [<Q $precision>]<i16>;
            )*
        }
    };
} 

variant_i16_!(1, 2, 3, 4);

macro_rules! variant_i32_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision I32>] = [<Q $precision>]<i32>;
            )*
        }
    };
} 

variant_i32_!(
    1, 2, 3,
    4, 5, 6,
    7, 8, 9
);

macro_rules! variant_i64_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision I64>] = [<Q $precision>]<i64>;
            )*
        }
    };
} 

variant_i64_!(
    1, 2, 3,
    4, 5, 6,
    7, 8, 9,
    10, 11, 12,
    13, 14, 15,
    16, 17, 18
);

macro_rules! variant_i128_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision I128>] = [<Q $precision>]<i128>;
            )*
        }
    };
} 

variant_i128_!(
    1, 2, 3,
    4, 5, 6,
    7, 8, 9,
    10, 11, 12,
    13, 14, 15,
    16, 17, 18,
    19, 20, 21,
    22, 23, 24,
    25, 26, 27,
    28, 29, 30,
    31, 32, 33,
    34, 35, 36,
    37, 38
);

macro_rules! _generic {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision>]<T> = Default<$precision, T>;
            )*
        }
    };
} 

_generic!(
    1, 2, 3,
    4, 5, 6,
    7, 8, 9,
    10, 11, 12,
    13, 14, 15,
    16, 17, 18,
    19, 20, 21,
    22, 23, 24,
    25, 26, 27,
    28, 29, 30,
    31, 32, 33,
    34, 35, 36,
    37, 38
);

pub type Default<const A: usize, B> = Q<A, B, default_engine::DefaultEngine>;

pub type Result<T> = ::core::result::Result<T, Error>;

#[cfg(not(feature = "ink"))]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("This value is above the representable range for this type.")]
    Overflow,
    #[error("This value is below the representable range for this type.")]
    Underflow,
    #[error("Cannot divide by zero.")]
    DivisionByZero,
    #[error("This value is above or below the representable range.")]
    UnsupportedConversion,
    #[error("")]
    NegativeSquareRoot
}

#[cfg(feature = "ink")]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[cfg_attr(not(feature = "ink"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "ink"), derive(serde::Deserialize))]
#[cfg_attr(feature = "ink", derive(ink::scale::Encode))]
#[cfg_attr(feature = "ink", derive(ink::scale::Decode))]
#[cfg_attr(feature = "ink", derive(ink::scale_info::TypeInfo))]
#[cfg_attr(feature = "ink", derive(ink::storage::traits::StorageLayout))]
pub enum Error {
    Overflow,
    Underflow,
    DivByZero,
    RemByZero,
    NegSqrRoot
}

#[derive(Clone)]
#[derive(Copy)]
#[cfg_attr(not(feature = "ink"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "ink"), derive(serde::Deserialize))]
#[cfg_attr(feature = "ink", derive(ink::scale::Encode))]
#[cfg_attr(feature = "ink", derive(ink::scale::Decode))]
#[cfg_attr(feature = "ink", derive(ink::scale_info::TypeInfo))]
#[cfg_attr(feature = "ink", derive(ink::storage::traits::StorageLayout))]
pub struct Q<const A: usize, B, C> 
where 
    B: int::Introspection,
    C: Engine,
    precision::Precision<A>: precision::Compatible {
    _v: B,
    _engine: C
}

pub fn new<const A: usize, B>(value: B) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Introspection,
    precision::Precision<A>: precision::Compatible {
    Q {
        _v: value,
        _engine: default_engine::new()
    }
}

pub fn new_with_engine_from<const A: usize, B, C>(value: B, sample: Q<A, B, C>) -> Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine,
    precision::Precision<A>: precision::Compatible {
    new_with_custom_engine(value, sample._engine)
}

pub fn new_with_custom_engine<const A: usize, B, C>(value: B, engine: C) -> Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine,
    precision::Precision<A>: precision::Compatible {
    Q {
        _v: value,
        _engine: engine
    }
}

pub fn from_int<
    const A: usize,
    const B: usize,
    C,
    D>(value: C) -> Result<Default<B, D>> 
where
    C: int::Int,
    C: int::Introspection,
    D: int::Int,
    D: int::Introspection,
    precision::Precision<A>: precision::Compatible,
    precision::Precision<B>: precision::Compatible {
    from_int_with_custom_engine::<A, B, C, D, default_engine::DefaultEngine>(value, default_engine::DefaultEngine)
}

pub fn from_int_with_engine_from<
    const A: usize,
    const B: usize,
    C,
    D,
    E,
    const F: usize,
    G>(value: C, sample: Q<F, G, E>) -> Result<Q<B, D, E>> 
where
    C: int::Int,
    C: int::Introspection,
    D: int::Int,
    D: int::Introspection,
    E: Engine,
    G: int::Int,
    G: int::Introspection,
    precision::Precision<A>: precision::Compatible,
    precision::Precision<B>: precision::Compatible,
    precision::Precision<F>: precision::Compatible {
    from_int_with_custom_engine::<A, B, C, D, E>(value, sample._engine)
}

pub fn from_int_with_custom_engine<
    const A: usize,
    const B: usize, 
    C, 
    D,
    E>(value: C, engine: E) -> Result<Q<B, D, E>> 
where 
    C: int::Int,
    C: int::Introspection,
    D: int::Int,
    D: int::Introspection,
    E: Engine,
    precision::Precision<A>: precision::Compatible,
    precision::Precision<B>: precision::Compatible {
    let value: D = D::from(value).ok_or(Error::UnsupportedConversion)?;
    let value: Q<A, D, E> = new_with_custom_engine(value, engine);
    let value: Q<B, D, E> = value.cast()?;
    Ok(value)
}



#[cfg(feature = "float")]
pub fn from_f32<const A: usize, B>(value: f32) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int::Introspection,
    precision::Precision<A>: precision::Compatible {
    from_f32_with_custom_engine(value, default_engine::new())
}

#[cfg(feature = "float")]
pub fn from_f32_with_custom_engine<const A: usize, B, C>(value: f32, engine: C) -> Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine,
    precision::Precision<A>: precision::Compatible {
    let scale = precision::SCALE_U128[A];
    let scale = (value * (scale as f32)).round();
    let value = B::from(scale).unwrap();
    Q {
        _v: value,
        _engine: engine
    }
}

#[cfg(feature = "float")]
pub fn from_f64<const A: usize, B>(value: f64) -> Default<A, B>
where
    B: int::Int,
    B: int::Introspection,
    precision::Precision<A>: precision::Compatible {
    
}

#[cfg(feature = "float")]
pub fn from_f64_with_custom_engine<const A: usize, B, C>() {

}

pub fn pi_with_custom_engine<const A: usize, B, C>(engine: C) -> Result<Q<A, B, C>>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine,
    precision::Precision<A>: precision::Compatible {
    if B::zero().is_signed() {
        let pi: i128 = trig::PI_I128[A];
        let pi: B = B::from(pi).ok_or(Error::UnsupportedConversion)?;
        let pi: Q<A, B, C> = new_with_custom_engine(pi, engine);
        return Ok(pi);
    }
    let pi: u128 = trig::PI_U128[A];
    let pi: B = B::from(pi).ok_or(Error::UnsupportedConversion)?;
    let pi: Q<A, B, C> = new_with_custom_engine(pi, engine);
    Ok(pi)
}

pub fn pi<const A: usize, B>() -> Default<A, B> 
where
    B: int::Int,
    B: int::Introspection,
    precision::Precision<A>: precision::Compatible {
    if B::zero().is_signed() {
        let pi: i128 = trig::PI_I128[A];
        let pi: B = B::from(pi).unwrap();
        let pi: Default<A, B> = new(pi);
        return pi;
    }
    let pi: u128 = trig::PI_U128[A];
    let pi: B = B::from(pi).unwrap();
    let pi: Default<A, B> = new(pi);
    pi
}

toga::blockset! {
    impl<const A: usize, B, C> Q<A, B, C> 
    where
        B: int::Int,
        B: int::Introspection,
        C: Engine,
        precision::Precision<A>: precision::Compatible;

    pub fn cast<const D: usize>(&self) -> Result<Q<D, B, C>> where precision::Precision<D>: precision::Compatible {
        self._engine.cast(self)
    }

    pub fn transform<const D: usize, E>(&self) -> Result<Q<D, E, C>>
    where
        E: int::Int,
        E: int::Introspection,
        precision::Precision<D>: precision::Compatible {
        let value: Q<D, B, C> = self.cast()?;
        let value: B = value._v;
        let value: E = E::from(value).ok_or(Error::UnsupportedConversion)?;
        let value: Q<D, E, C> = new_with_custom_engine(value, self._engine);
        Ok(value)
    }

    pub fn cos(&self) -> Result<Self> {
        self._engine.cos(self)
    }

    pub fn sin(&self) -> Result<Self> {
        self._engine.sin(self)
    }

    pub fn tan(&self) -> Result<Self> {
        self._engine.tan(self)
    }

    pub fn sqrt(&self) -> Result<Self> {
        self._engine.sqrt(self)
    }

    pub fn to_radians(&self) -> Result<Self> {
        (pi_with_custom_engine(self._engine)? * *self)? / from_int_with_custom_engine(180, self._engine)?
    }

    pub fn to_degrees(&self) -> Result<Self> {
        (from_int_with_custom_engine(180, self._engine)? * *self)? / pi_with_custom_engine(self._engine)?
    }

    pub fn to_i8(&self) -> Result<i8> {
        let result: i8 = self._v.to_i8().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_i16(&self) -> Result<i16> {
        let result: i16 = self._v.to_i16().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_i32(&self) -> Result<i32> {
        let result: i32 = self._v.to_i32().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_i64(&self) -> Result<i64> {
        let result: i64 = self._v.to_i64().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_i128(&self) -> Result<i128> {
        let result: i128 = self._v.to_i128().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_isize(&self) -> Result<isize> {
        let result: isize = self._v.to_isize().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_u8(&self) -> Result<u8> {
        let result: u8 = self._v.to_u8().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_u16(&self) -> Result<u16> {
        let result: u16 = self._v.to_u16().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_u32(&self) -> Result<u32> {
        let result: u32 = self._v.to_u32().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_u64(&self) -> Result<u64> {
        let result: u64 = self._v.to_u64().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    pub fn to_u128(&self) -> Result<u128> {
        let result: u128 = self._v.to_u128().ok_or(Error::UnsupportedConversion)?;
        Ok(result)
    }

    ops::Add {
        type Output = Result<Self>;

        fn add(self, rhs: Self) -> Self::Output {
            debug_assert!(A >= 1);
            debug_assert!(A <= 38);
            let x: &Self = &self;
            let y: &Self = &rhs;
            self._engine.add(x, y)
        }
    }

    ops::Sub {
        type Output = Result<Self>;

        fn sub(self, rhs: Self) -> Self::Output {
            debug_assert!(A >= 1);
            debug_assert!(A <= 38);
            let x: &Self = &self;
            let y: &Self = &rhs;
            self._engine.sub(x, y)
        }
    }

    ops::Mul {
        type Output = Result<Self>;

        fn mul(self, rhs: Self) -> Self::Output {
            debug_assert!(A >= 1);
            debug_assert!(A <= 38);
            let x: &Self = &self;
            let y: &Self = &rhs;
            self._engine.mul(x, y)
        }
    }

    ops::Div {
        type Output = Result<Self>;

        fn div(self, rhs: Self) -> Self::Output {
            debug_assert!(A >= 1);
            debug_assert!(A <= 38);
            let x: &Self = &self;
            let y: &Self = &rhs;
            self._engine.div(x, y)
        }
    }

    Ord {
        fn clamp(self, min: Self, max: Self) -> Self {
            if self > max {
                return max;
            }
            if self < min {
                return min;
            }
            self
        }
    
        fn max(self, other: Self) -> Self {
            if self > other {
                return self;
            }
            other
        }
    
        fn min(self, other: Self) -> Self {
            if self < other {
                return self;
            }
            other
        }
    
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            if self > other {
                return cmp::Ordering::Greater
            }
            if self < other {
                return cmp::Ordering::Less
            }
            cmp::Ordering::Equal
        }
    }

    PartialOrd {
        fn ge(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v >= y._v
        }
    
        fn le(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v <= y._v
        }
    
        fn gt(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v > y._v
        }
    
        fn lt(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v < y._v
        }
    
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    PartialEq {
        fn eq(&self, other: &Self) -> bool {
            let x: &Self = self;
            let x: &B = &x._v;
            let y: &Self = other;
            let y: &B = &y._v;
            x == y
        }
    }

    Eq {}
}

#[cfg(not(feature = "float"))]
toga::blockset! {
    impl<const A: usize, B, C> Q<A, B, C> 
    where
        B: int::Int,
        B: int::Introspection,
        B: fmt::Debug,
        B: fmt::Display,
        C: Engine,
        precision::Precision<A>: precision::Compatible;

    fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self._v)
        }
    }

    fmt::Display {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self._v)
        }
    }
}

#[cfg(feature = "float")]
toga::blockset! {
    impl<const A: usize, B, C> Q<A, B, C> 
    where
        B: int::Int,
        B: int::Introspection,
        C: Engine,
        precision::Precision<A>: precision::Compatible;

    pub fn to_f64(&self) -> Result<f64> {
        if B::zero().is_signed() {
            let scale: f64 = precision::SCALE_I128[A].to_f64().ok_or(Error::UnsupportedConversion)?;
            let value: f64 = self._v.to_i128().unwrap().to_f64().ok_or(Error::UnsupportedConversion)?;
            return Ok(value / scale)
        }
        let scale: f64 = precision::SCALE_U128[A].to_f64().ok_or(Error::UnsupportedConversion)?;
        let value: f64 = self._v.to_f64().ok_or(Error::UnsupportedConversion)?;
        Ok(value / scale)
    }

    pub fn to_f32(&self) -> Result<f32> {
        if B::zero().is_signed() {
            let scale: f32 = precision::SCALE_I128[A].to_f32().ok_or(Error::UnsupportedConversion)?;
            let value: f32 = self._v.to_i128().unwrap().to_f32().ok_or(Error::UnsupportedConversion)?;
            return Ok(value / scale)
        }
        let scale: f32 = precision::SCALE_U128[A].to_f32().ok_or(Error::UnsupportedConversion)?;
        let value: f32 = self._v.to_u128().unwrap().to_f32().ok_or(Error::UnsupportedConversion)?;
        Ok(value / scale)
    }

    fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let value: f32 = self.to_f32().unwrap();
            write!(f, "{}", value)
        }
    }

    fmt::Display {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let value: f32 = self.to_f32().unwrap();
            write!(f, "{}", value)
        }
    }
}

toga::blockset! {
    impl<const A: usize, B, C> Q<A, B, C> 
    where
        B: int::Int,
        B: int::Introspection,
        B: num_traits::WrappingNeg,
        C: Engine,
        precision::Precision<A>: precision::Compatible;

    pub fn abs(&self) -> Result<Self> {
        if self._v.is_signed() && self._v.to_i128().unwrap() < 0 {
            let value: B = self._v.wrapping_neg();
            let value: Self = new_with_custom_engine(value, self._engine);
            return Ok(value)
        }
        Ok(*self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[rstest::rstest]
    #[case(new(100), new(100), new::<2, u128>(200))]
    pub fn add<const A: usize, B>(
        #[case] x: Default<A, B>,
        #[case] y: Default<A, B>,
        #[case] ok: Default<A, B>
    ) 
    where 
        B: int::Int,
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        let result: Default<A, B> = (x + y).unwrap();
        assert_eq!(result, ok);
    }

    #[rstest::rstest]
    #[case(new(100), new(100), new::<2, u128>(0))]
    pub fn sub<const A: usize, B>(
        #[case] x: Default<A, B>,
        #[case] y: Default<A, B>,
        #[case] ok: Default<A, B>
    )
    where 
        B: int::Int,
        B: int::Introspection,
        precision::Precision<A>: precision::Compatible {
        let result: Default<A, B> = (x - y).unwrap();
        assert_eq!(result, ok);
    }
}