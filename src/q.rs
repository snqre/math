use crate::int;
use crate::precision;
use crate::default_engine;
use crate::math;
use crate::trig;

use ::core::ops;
use ::core::fmt;
use ::core::cmp;

use int::Introspection as _;

pub trait Engine 
where
    Self: Sized,
    Self: Clone,
    Self: Copy {

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
            let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let scale: u128 = precision::SCALE_U128[A];
        let result: u128 = self.muldiv(&x, &y, &scale)?;
        let result: B = B::from(result).unwrap();
        let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
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
                let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
                return Ok(result)
            }
            let scale: i128 = precision::SCALE_I128[A];
            let result: i128 = self.muldiv(&x, &scale, &y)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        if scale.is_power_of_two() {
            let z: u32 = scale.trailing_zeros();
            let result: u128 = (x << z).checked_div(y).ok_or(Error::DivisionByZero)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
            return Ok(result)
        }
        let result: u128 = self.muldiv(&x, &scale, &y)?;
        let result: B = B::from(result).unwrap();
        let result: Q<A, B, Self> = new_with_custom_engine(result, self.clone());
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

macro_rules! variant_u8_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision U8>] = [<Q $precision>]<u8>;
            )*
        }
    };
}

variant_u8_!(1, 2);

macro_rules! variant_u16_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision U16>] = [<Q $precision>]<u16>;
            )*
        }
    };
}

variant_u16_!(1, 2, 3, 4);

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

macro_rules! variant_generic_ {
    ($($precision:literal),*) => {
        paste::paste! {
            $(
                pub type [<Q $precision>]<T> = QDefault<$precision, T>;
            )*
        }
    };
} 

variant_generic_!(
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

pub type QDefault<const A: usize, B> = Q<A, B, default_engine::DefaultEngine>;

pub type Result<T> = ::core::result::Result<T, Error>;

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

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(Clone)]
#[derive(Copy)]
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

pub fn from_f32<const A: usize, B>(value: f32) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int::Introspection,
    precision::Precision<A>: precision::Compatible {
    from_f32_with_custom_engine(value, default_engine::new())
}

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

pub fn from_f64_with_custom_engine<const A: usize, B, C>() {

}

toga::blockset! {
    impl<const A: usize, B, C> Q<A, B, C> where
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
        let value: Q<D, B> = self.cast()?;
        let value: B = value._v;
        let value: E = E::from(value).ok_or(Error::UnsupportedConversion)?;
        let value: Q<D, E> = new_with_custom_engine(value, value._engine);
        Ok(value)
    }

    pub fn abs(&self) -> Result<Self> {
        if self._v.is_signed() && self._v.to_i128().unwrap() < 0 {
            let value: B = self._v.wrapping_neg();
            let value: Self = new_with_custom_engine(value, self._engine);
            return Ok(value)
        }
        Ok(self)
    }

    pub fn cos(&self) -> Result<Self> {
        
    }

    pub fn sin(&self) -> Result<Self> {

    }

    pub fn tan(&self) -> Result<Self> {

    }

    pub fn pow(&self) -> Result<Self> {

    }

    fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let value: f32 = self.try_into()?;
            write!(f, "{}", value)
        }
    }

    fmt::Display {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let value: f32 = self.try_into()?;
            write!(f, "{}", value)
        }
    }

    ops::Add {
        type Output = Result<Self>;

        fn add(self, rhs: Self) -> Self::Output {
            debug_assert!(A >= 1);
            debug_assert!(A <= 38);
            let x: &Self = &self;
            let x: &B = &x._v;
            let y: &Self = &rhs;
            let y: &B = &y._v;
            let result: B = x.checked_add(y).ok_or(Error::Overflow)?;
            Ok(new_with_custom_engine(result, self._engine))
        }
    }

    ops::Sub {
        type Output = Result<Self>;

        fn sub(self, rhs: Self) -> Self::Output {
            debug_assert!(A >= 1);
            debug_assert!(A <= 38);
            let x: &Self = &self;
            let x: &B = &x._v;
            let y: &Self = &rhs;
            let y: &B = &y._v;
            let result: B = x.checked_sub(y).ok_or(Error::Underflow)?;
            Ok(new_with_custom_engine(result, self._engine))
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




macro_rules! try_from_and_try_into_int_ {
    ($($size:ty),*) => {
        paste::paste! {
            $(
                impl<const A: usize, B> TryFrom<$size> for Q<A, B, default_engine::DefaultEngine> 
                where
                    B: int::Int,
                    B: int::Introspection,
                    precision::Precision<A>: precision::Compatible {
                    
                    type Error = Error;
                
                    fn try_from(value: $size) -> core::result::Result<Self, Self::Error> {
                        let value: B = B::from(value).ok_or(Error::UnsupportedConversion)?;
                        Ok(new(value))
                    }
                }
    
                impl<const A: usize, B> TryInto<$size> for Q<A, B, default_engine::DefaultEngine> 
                where
                    B: int::Int,
                    B: int::Introspection,
                    precision::Precision<A>: precision::Compatible {
                    
                    type Error = Error;
                
                    fn try_into(self) -> core::result::Result<$size, Self::Error> {
                        let value: $size = self._v.[<to_ $size>]().ok_or(Error::UnsupportedConversion)?;
                        Ok(value)
                    }
                }
            )*
        }
    };
} 

try_from_and_try_into_int_!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

macro_rules! try_from_and_try_into_float_ {
    ($($size:ty),*) => {
        paste::paste! {
            $(
                impl<const A: usize, B> TryFrom<$size> for Q<A, B, default_engine::DefaultEngine> 
                where
                    B: int::Int,
                    B: int::Introspection,
                    precision::Precision<A>: precision::Compatible {
                    
                    type Error = Error;
                
                    fn try_from(value: $size) -> core::result::Result<Self, Self::Error> {
                        let scale: u128 = precision::SCALE_U128[A];
                        let scale: $size = scale.into();
                        let value_scaled: $size = (value * scale).round();
                        if B::zero().is_signed() {
                            let value: i128 = value_scaled.into();
                            let value: B = B::from(value).ok_or(Error::UnsupportedConversion)?;
                            return Ok(new(value))
                        }
                        if value_scaled < 0.0 {
                            return Err(Error::UnsupportedConversion)
                        }
                        let value: u128 = value_scaled.into();
                        let value: B = B::from(value).ok_or(Error::UnsupportedConversion)?;
                        Ok(new(value))
                    }
                }
                
                impl<const A: usize, B> TryInto<$size> for Q<A, B, default_engine::DefaultEngine> 
                where
                    B: int::Int,
                    B: int::Introspection,
                    precision::Precision<A>: precision::Compatible {
                    
                    type Error = Error;
                
                    fn try_into(self) -> core::result::Result<$size, Self::Error> {
                        if B::zero().is_signed() {
                            let scale: $size = precision::SCALE_I128[A].into();
                            let value: $size = self._v.to_i128().unwrap().into();
                            return Ok(value / scale)
                        }
                        let scale: $size = precision::SCALE_U128[A].into();
                        let value: $size = self._v.to_u128().unwrap().into();
                        Ok(value / scale)
                    }
                }
            )*
        }
    };
} 

try_from_and_try_into_float_!(f32, f64);