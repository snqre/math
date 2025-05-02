use crate::ambient::universal::Universal;
use crate::ambient::int::Int;
use crate::ambient::int::introspection::Introspection;
use crate::ambient::int::introspection::Type;
use crate::engine::default_engine::DefaultEngine;
use ::core::ops::Add;
use ::core::ops::Sub;
use ::core::ops::Mul;
use ::core::ops::Div;
use ::core::ops::Rem;
use ::core::cmp::Ordering;
use num_traits::CheckedRem;
use num_traits::Signed;
use num_traits::ToPrimitive;
use num_traits::Unsigned;
use num_traits::WrappingNeg;
use num_traits::Zero;
use ::thiserror::Error;

// data -> assoc
// main
// impl
// adaptor
// service
// port

pub type Result<T> = ::core::result::Result<T, Error>;
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
    DivByZero,
    #[error("")]
    RemByZero,
    #[error("")]
    UnsupportedConversion
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B, C>
    where
        B: Int,
        B: Introspection,
        C: Engine {
    pub(crate) _v: B,
    pub(crate) _engine: C
}

pub fn new<const A: u8, B>(v: B) -> Q<A, B, DefaultEngine> 
    where
        B: Int,
        B: Introspection {
    Q {
        _v: v,
        _engine: DefaultEngine
    }
}

pub fn new_with_custom_engine<const A: u8, B, C>(v: B, engine: C) -> Q<A, B, C> 
    where
        B: Int,
        B: Introspection,
        C: Engine {
    Q {
        _v: v,
        _engine: engine
    }
}

impl<const A: u8, B, C> Rem for Q<A, B, C> 
    where
        B: Int,
        B: Introspection,
        B: CheckedRem,
        C: Engine {

    type Output = Result<Self>;

    fn rem(self, rhs: Self) -> <Self as Rem>::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.rem(x, y)
    }
}

impl<const A: u8, B, C> Add for Q<A, B, C> 
    where
        B: Int,
        B: Introspection,
        C: Engine {

    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.add(x, y)
    }
}

impl<const A: u8, B, C> Sub for Q<A, B, C> 
    where
        B: Int,
        B: Introspection,
        C: Engine {

    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.sub(x, y)
    }
}

macro_rules! muldiv {
    ($(($precision:expr, $size:ty)),*) => {
        $(
            impl<T> Mul for Q<$precision, $size, T>
                where
                    T: Engine {

                type Output = Result<Self>;
            
                fn mul(self, rhs: Self) -> Self::Output {
                    let x: &Self = &self;
                    let y: &Self = &rhs;
                    self._engine.mul(x, y)
                }
            }

            impl<T> Div for Q<$precision, $size, T>
                where
                    T: Engine {

                type Output = Result<Self>;
            
                fn div(self, rhs: Self) -> Self::Output {
                    let x: &Self = &self;
                    let y: &Self = &rhs;
                    self._engine.div(x, y)
                }
            }
        )*
    };
} 

muldiv!(
    (1, u8),
    (2, u8),
    (1, u16),
    (2, u16),
    (3, u16),
    (4, u16),
    (1, u32),
    (2, u32),
    (3, u32),
    (4, u32),
    (5, u32),
    (6, u32),
    (7, u32),
    (8, u32),
    (9, u32),
    (1, u64),
    (2, u64),
    (3, u64),
    (4, u64),
    (5, u64),
    (6, u64),
    (7, u64),
    (8, u64),
    (9, u64),
    (10, u64),
    (11, u64),
    (12, u64),
    (13, u64),
    (14, u64),
    (15, u64),
    (16, u64),
    (17, u64),
    (18, u64),
    (19, u64),
    (1, u128),
    (2, u128),
    (3, u128),
    (4, u128),
    (5, u128),
    (6, u128),
    (7, u128),
    (8, u128),
    (9, u128),
    (10, u128),
    (11, u128),
    (12, u128),
    (13, u128),
    (14, u128),
    (15, u128),
    (16, u128),
    (17, u128),
    (18, u128),
    (19, u128),
    (20, u128),
    (21, u128),
    (22, u128),
    (23, u128),
    (24, u128),
    (25, u128),
    (26, u128),
    (27, u128),
    (28, u128),
    (29, u128),
    (30, u128),
    (31, u128),
    (32, u128),
    (33, u128),
    (34, u128),
    (35, u128),
    (36, u128),
    (37, u128),
    (38, u128),
    (1, i8),
    (2, i8),
    (1, i16),
    (2, i16),
    (3, i16),
    (4, i16),
    (1, i32),
    (2, i32),
    (3, i32),
    (4, i32),
    (5, i32),
    (6, i32),
    (7, i32),
    (8, i32),
    (9, i32),
    (1, i64),
    (2, i64),
    (3, i64),
    (4, i64),
    (5, i64),
    (6, i64),
    (7, i64),
    (8, i64),
    (9, i64),
    (10, i64),
    (11, i64),
    (12, i64),
    (13, i64),
    (14, i64),
    (15, i64),
    (16, i64),
    (17, i64),
    (18, i64),
    (1, i128),
    (2, i128),
    (3, i128),
    (4, i128),
    (5, i128),
    (6, i128),
    (7, i128),
    (8, i128),
    (9, i128),
    (10, i128),
    (11, i128),
    (12, i128),
    (13, i128),
    (14, i128),
    (15, i128),
    (16, i128),
    (17, i128),
    (18, i128),
    (19, i128),
    (20, i128),
    (21, i128),
    (22, i128),
    (23, i128),
    (24, i128),
    (25, i128),
    (26, i128),
    (27, i128),
    (28, i128),
    (29, i128),
    (30, i128),
    (31, i128),
    (32, i128),
    (33, i128),
    (34, i128),
    (35, i128),
    (36, i128),
    (37, i128),
    (38, i128)
);

impl<const A: u8, B, C> Ord for Q<A, B, C> 
    where
        B: Int,
        B: Introspection,
        C: Engine {

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

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const A: u8, B, C> PartialOrd for Q<A, B, C> 
    where
        B: Int,
        B: Introspection,
        C: Engine {

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

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into_some()
    }
}

impl<const A: u8, B, C> Eq for Q<A, B, C> 
    where
        B: Int,
        B: Introspection,
        C: Engine {}

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
    where
        B: Int,
        B: Introspection,
        C: Engine {

    fn eq(&self, other: &Self) -> bool {
        let x: &Self = self;
        let x: &B = &x._v;
        let y: &Self = other;
        let y: &B = &y._v;
        x == y
    }
}

pub trait Engine    
    where
        Self: Sized,
        Self: Clone {

    fn cast<const A: u8, const B: u8, C>(&self, x: &Q<A, C, Self>) -> Result<Q<B, C, Self>> 
        where
            C: Int,
            C: Introspection {

        let x: C = x._v;
        if A == B {
            let engine: Self = self.clone();
            let r: C = x;
            let r: Q<B, C, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let old_precision: u32 = A.into();
        let new_precision: u32 = B.into();
        if x.is_signed() {
            let base: i128 = 10;
            let old_scale: i128 = base.pow(old_precision);
            let new_scale: i128 = base.pow(new_precision);
            let engine: Self = self.clone();
            let r: i128 = x.to_i128().unwrap();
            let r: i128 = self.muldiv(&r, &new_scale, &old_scale)?;
            let r: C = C::from(r).unwrap();
            let r: Q<B, C, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let base: u128 = 10;
        let old_scale: u128 = base.pow(old_precision);
        let new_scale: u128 = base.pow(new_precision);
        let engine: Self = self.clone();
        let r: u128 = x.to_u128().unwrap();
        let r: u128 = self.muldiv(&r, &new_scale, &old_scale)?;
        let r: C = C::from(r).unwrap();
        let r: Q<B, C, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn mul<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
        where
            B: Int,
            B: Introspection {

        let x: B = x._v;
        let y: B = y._v;
        let precision: u32 = A.into();
        if x.is_signed() {
            let base: i128 = 10;
            let scale: i128 = base.pow(precision);
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            let engine: Self = self.clone();
            let r: i128 = self.muldiv(&x, &y, &scale)?;
            let r: B = B::from(r).unwrap();
            let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let base: u128 = 10;
        let scale: u128 = base.pow(precision);
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let r: u128 = self.muldiv(&x, &y, &scale)?;
        let engine: Self = self.clone();
        let r: B = B::from(r).unwrap();
        let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn div<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
        where
            B: Int,
            B: Introspection {

        let x: B = x._v;
        let y: B = y._v;
        let precision: u32 = A.into();
        let base: u128 = 10;
        let scale: u128 = base.pow(precision);
        if x.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            if scale.is_power_of_two() {
                let z: u32 = scale.trailing_zeros();
                let engine: Self = self.clone();
                let r: i128 = (x << z).checked_div(y).ok_or(Error::DivByZero)?;
                let r: B = B::from(r).unwrap();
                let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
                return r.into_ok()
            }
            let scale: i128 = scale.to_i128().unwrap();
            let engine: Self = self.clone();
            let r: i128 = self.muldiv(&x, &scale, &y)?;
            let r: B = B::from(r).unwrap();
            let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        if scale.is_power_of_two() {
            let z: u32 = scale.trailing_zeros();
            let engine: Self = self.clone();
            let r: u128 = (x << z).checked_div(y).ok_or(Error::DivByZero)?;
            let r: B = B::from(r).unwrap();
            let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let engine: Self = self.clone();
        let r: u128 = self.muldiv(&x, &scale, &y)?;
        let r: B = B::from(r).unwrap();
        let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn rem<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
        where
            B: Int,
            B: Introspection,
            B: CheckedRem {
                
        let x: B = x._v;
        let y: B = y._v;
        if y == B::zero() {
            return Error::DivByZero.into_err()
        }
        let r: B = x.checked_rem(&y).ok_or(Error::RemByZero)?;
        let engine: Self = self.clone();
        let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn add<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
        where 
            B: Int,
            B: Introspection {

        let x: &Q<A, B, Self> = x;
        let x: B = x._v;
        let y: &Q<A, B, Self> = y;
        let y: B = y._v;
        let z: B = x.checked_add(&y).ok_or(Error::Overflow)?;
        let engine: Self = self.clone();
        let z: Q<A, B, Self> = new_with_custom_engine(z, engine);
        z.into_ok()
    }

    fn sub<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
        where
            B: Int,
            B: Introspection {

        let x: &Q<A, B, Self> = x;
        let x: B = x._v;
        let y: &Q<A, B, Self> = y;
        let y: B = y._v;
        let z: B = x.checked_sub(&y).ok_or(Error::Overflow)?;
        let engine: Self = self.clone();
        let z: Q<A, B, Self> = new_with_custom_engine(z, engine);
        z.into_ok()
    }

    fn muldiv<'a, T>(&self, x: &'a T, y: &'a T, z: &'a T) -> Result<T>
        where
            T: Int,
            T: Introspection {

        if z == &T::zero() {
            return Error::DivByZero.into_err()
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
                return Error::Overflow.into_err()
            }
            if b == 0 {
                let r: u128 = a / z;
                if sign == 1 {
                    let r: i128 = r.wrapping_neg().try_into().unwrap();
                    let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
                    return r.into_ok()
                }
                let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
                return r.into_ok()
            }
            let r: u128 = _fold_128_bit_product_mod(a, b, z) / z;
            if sign == 1 {
                let r: i128 = r.wrapping_neg().try_into().unwrap();
                let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
                return r.into_ok()
            }
            let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
            return r.into_ok()
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let z: u128 = z.to_u128().unwrap();
        let (a, b) = _wide_mul(x, y);
        if b >= z {
            return Error::Overflow.into_err()
        }
        if b == 0 {
            let r: u128 = a / z;
            let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
            return r.into_ok()
        }
        let r: u128 = _fold_128_bit_product_mod(a, b, z) / z;
        let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
        r.into_ok()
    }
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