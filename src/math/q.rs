use crate::common::int;
use crate::common::util;
use crate::math::engine::default_engine;
use ::core::ops;
use ::core::cmp;
use ::thiserror as error;
use ::num_traits as num;

use util::Util as _;
use num::ToPrimitive as _;
use num::FromPrimitive as _;
use int::Introspection as _;
use int::Int as _;

pub trait Engine    
where
    Self: Sized,
    Self: Clone {
    fn cast<const A: u8, const B: u8, C>(&self, x: &Q<A, C, Self>) -> Result<Q<B, C, Self>> 
    where
        C: int::Int,
        C: int::Introspection {
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
        B: int::Int,
        B: int::Introspection {
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
        B: int::Int,
        B: int::Introspection {
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
        B: int::Int,
        B: int::Introspection,
        B: num::CheckedRem {
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
        B: int::Int,
        B: int::Introspection {
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
        B: int::Int,
        B: int::Introspection {
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
        T: int::Int,
        T: int::Introspection {
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
                    let r: T = r.to_i128()
                        .ok_or(Error::UnsupportedConversion)?
                        .wrapping_neg()
                        .to_int()
                        .ok_or(Error::UnsupportedConversion)?;
                    return r.into_ok()
                }
                let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
                return r.into_ok()
            }
            let r: u128 = _fold_128_bit_product_mod(a, b, z) / z;
            if sign == 1 {
                let r: T = r.to_i128()
                    .ok_or(Error::UnsupportedConversion)?
                    .wrapping_neg()
                    .to_int()
                    .ok_or(Error::UnsupportedConversion)?;
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

pub type Q1U8 = Q1<u8>;
pub type Q2U8 = Q2<u8>;

pub type Q1U16 = Q1<u16>;
pub type Q2U16 = Q2<u16>;
pub type Q3U16 = Q3<u16>;
pub type Q4U16 = Q4<u16>;

pub type Q1U32 = Q1<u32>;
pub type Q2U32 = Q2<u32>;
pub type Q3U32 = Q3<u32>;
pub type Q4U32 = Q4<u32>;
pub type Q5U32 = Q5<u32>;
pub type Q6U32 = Q6<u32>;
pub type Q7U32 = Q7<u32>;
pub type Q8U32 = Q8<u32>;
pub type Q9U32 = Q9<u32>;

pub type Q1U64 = Q1<u64>;
pub type Q2U64 = Q2<u64>;
pub type Q3U64 = Q3<u64>;
pub type Q4U64 = Q4<u64>;
pub type Q5U64 = Q5<u64>;
pub type Q6U64 = Q6<u64>;
pub type Q7U64 = Q7<u64>;
pub type Q8U64 = Q8<u64>;
pub type Q9U64 = Q9<u64>;

pub type Q1U128 = Q1<u128>;
pub type Q2U128 = Q2<u128>;
pub type Q3U128 = Q3<u128>;
pub type Q4U128 = Q4<u128>;
pub type Q5U128 = Q5<u128>;
pub type Q6U128 = Q6<u128>;
pub type Q7U128 = Q7<u128>;
pub type Q8U128 = Q8<u128>;
pub type Q9U128 = Q9<u128>;
pub type Q10U128 = Q10<u128>;
pub type Q11U128 = Q11<u128>;
pub type Q12U128 = Q12<u128>;
pub type Q13U128 = Q13<u128>;
pub type Q14U128 = Q14<u128>;
pub type Q15U128 = Q15<u128>;
pub type Q16U128 = Q16<u128>;
pub type Q17U128 = Q17<u128>;
pub type Q18U128 = Q18<u128>;
pub type Q19U128 = Q19<u128>;
pub type Q20U128 = Q20<u128>;
pub type Q21U128 = Q21<u128>;
pub type Q22U128 = Q22<u128>;
pub type Q23U128 = Q23<u128>;
pub type Q24U128 = Q24<u128>;
pub type Q25U128 = Q25<u128>;
pub type Q26U128 = Q26<u128>;
pub type Q27U128 = Q27<u128>;
pub type Q28U128 = Q28<u128>;
pub type Q29U128 = Q29<u128>;
pub type Q30U128 = Q30<u128>;
pub type Q31U128 = Q31<u128>;
pub type Q32U128 = Q32<u128>;
pub type Q33U128 = Q33<u128>;
pub type Q34U128 = Q34<u128>;
pub type Q35U128 = Q35<u128>;
pub type Q36U128 = Q36<u128>;
pub type Q37U128 = Q37<u128>;
pub type Q38U128 = Q38<u128>;

pub type Q1I8 = Q1<i8>;
pub type Q2I8 = Q2<i8>;

pub type Q1I16 = Q1<i16>;
pub type Q2I16 = Q2<i16>;
pub type Q3I16 = Q3<i16>;
pub type Q4I16 = Q4<i16>;

pub type Q1I32 = Q1<i32>;
pub type Q2I32 = Q2<i32>;
pub type Q3I32 = Q3<i32>;
pub type Q4I32 = Q4<i32>;
pub type Q5I32 = Q5<i32>;
pub type Q6I32 = Q6<i32>;
pub type Q7I32 = Q7<i32>;
pub type Q8I32 = Q8<i32>;
pub type Q9I32 = Q9<i32>;

pub type Q1I64 = Q1<i64>;
pub type Q2I64 = Q2<i64>;

pub type Q1I128 = Q1<i128>;
pub type Q2I128 = Q2<i128>;
pub type Q3I128 = Q3<i128>;
pub type Q4I128 = Q4<i128>;
pub type Q5I128 = Q5<i128>;
pub type Q6I128 = Q6<i128>;
pub type Q7I128 = Q7<i128>;
pub type Q8I128 = Q8<i128>;
pub type Q9I128 = Q9<i128>;
pub type Q10I128 = Q10<i128>;
pub type Q11I128 = Q11<i128>;
pub type Q12I128 = Q12<i128>;
pub type Q13I128 = Q13<i128>;
pub type Q14I128 = Q14<i128>;
pub type Q15I128 = Q15<i128>;
pub type Q16I128 = Q16<i128>;
pub type Q17I128 = Q17<i128>;
pub type Q18I128 = Q18<i128>;
pub type Q19I128 = Q19<i128>;
pub type Q20I128 = Q20<i128>;
pub type Q21I128 = Q21<i128>;
pub type Q22I128 = Q22<i128>;
pub type Q23I128 = Q23<i128>;
pub type Q24I128 = Q24<i128>;
pub type Q25I128 = Q25<i128>;
pub type Q26I128 = Q26<i128>;
pub type Q27I128 = Q27<i128>;
pub type Q28I128 = Q28<i128>;
pub type Q29I128 = Q29<i128>;
pub type Q30I128 = Q30<i128>;
pub type Q31I128 = Q31<i128>;
pub type Q32I128 = Q32<i128>;
pub type Q33I128 = Q33<i128>;
pub type Q34I128 = Q34<i128>;
pub type Q35I128 = Q35<i128>;
pub type Q36I128 = Q36<i128>;
pub type Q37I128 = Q37<i128>;
pub type Q38I128 = Q38<i128>;

pub type Q1<T> = Q<1, T, default_engine::DefaultEngine>;
pub type Q2<T> = Q<2, T, default_engine::DefaultEngine>;
pub type Q3<T> = Q<3, T, default_engine::DefaultEngine>;
pub type Q4<T> = Q<4, T, default_engine::DefaultEngine>;
pub type Q5<T> = Q<5, T, default_engine::DefaultEngine>;
pub type Q6<T> = Q<6, T, default_engine::DefaultEngine>;
pub type Q7<T> = Q<7, T, default_engine::DefaultEngine>;
pub type Q8<T> = Q<8, T, default_engine::DefaultEngine>;
pub type Q9<T> = Q<9, T, default_engine::DefaultEngine>;
pub type Q10<T> = Q<10, T, default_engine::DefaultEngine>;
pub type Q11<T> = Q<11, T, default_engine::DefaultEngine>;
pub type Q12<T> = Q<12, T, default_engine::DefaultEngine>;
pub type Q13<T> = Q<13, T, default_engine::DefaultEngine>;
pub type Q14<T> = Q<14, T, default_engine::DefaultEngine>;
pub type Q15<T> = Q<15, T, default_engine::DefaultEngine>;
pub type Q16<T> = Q<16, T, default_engine::DefaultEngine>;
pub type Q17<T> = Q<17, T, default_engine::DefaultEngine>;
pub type Q18<T> = Q<18, T, default_engine::DefaultEngine>;
pub type Q19<T> = Q<19, T, default_engine::DefaultEngine>;
pub type Q20<T> = Q<20, T, default_engine::DefaultEngine>;
pub type Q21<T> = Q<21, T, default_engine::DefaultEngine>;
pub type Q22<T> = Q<22, T, default_engine::DefaultEngine>;
pub type Q23<T> = Q<23, T, default_engine::DefaultEngine>;
pub type Q24<T> = Q<24, T, default_engine::DefaultEngine>;
pub type Q25<T> = Q<25, T, default_engine::DefaultEngine>;
pub type Q26<T> = Q<26, T, default_engine::DefaultEngine>;
pub type Q27<T> = Q<27, T, default_engine::DefaultEngine>;
pub type Q28<T> = Q<28, T, default_engine::DefaultEngine>;
pub type Q29<T> = Q<29, T, default_engine::DefaultEngine>;
pub type Q30<T> = Q<30, T, default_engine::DefaultEngine>;
pub type Q31<T> = Q<31, T, default_engine::DefaultEngine>;
pub type Q32<T> = Q<32, T, default_engine::DefaultEngine>;
pub type Q33<T> = Q<33, T, default_engine::DefaultEngine>;
pub type Q34<T> = Q<34, T, default_engine::DefaultEngine>;
pub type Q35<T> = Q<35, T, default_engine::DefaultEngine>;
pub type Q36<T> = Q<36, T, default_engine::DefaultEngine>;
pub type Q37<T> = Q<37, T, default_engine::DefaultEngine>;
pub type Q38<T> = Q<38, T, default_engine::DefaultEngine>;

pub type Result<T> = ::core::result::Result<T, Error>;
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(error::Error)]
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
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    _v: B,
    _engine: C
}

pub fn new<const A: u8, B>(v: B) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int::Introspection {
    Q {
        _v: v,
        _engine: default_engine::new()
    }
}

#[cfg(feature = "custom_math")]
pub fn new_with_custom_engine<const A: u8, B, C>(v: B, engine: C) -> Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    Q {
        _v: v,
        _engine: engine
    }
}

impl<const A: u8, B, C> ops::Rem for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    B: num::CheckedRem,
    C: Engine {
    type Output = Result<Self>;

    fn rem(self, rhs: Self) -> <Self as ops::Rem>::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.rem(x, y)
    }
}

impl<const A: u8, B, C> ops::Add for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.add(x, y)
    }
}

impl<const A: u8, B, C> ops::Sub for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.sub(x, y)
    }
}

impl<const A: u8, B, C> ops::Mul for Q<A, B, C> 
where 
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.mul(x, y)
    }
}

impl<const A: u8, B, C> ops::Div for Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.div(x, y)
    }
}

impl<const A: u8, B, C> Ord for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
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

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const A: u8, B, C> PartialOrd for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
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

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.cmp(other).into_some()
    }
}

impl<const A: u8, B, C> Eq for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {}

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {

    fn eq(&self, other: &Self) -> bool {
        let x: &Self = self;
        let x: &B = &x._v;
        let y: &Self = other;
        let y: &B = &y._v;
        x == y
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    use crate::common::int;
    use ::rstest::rstest;
    use ::core::fmt;

    #[rstest]
    #[case(new(5), new(50), new::<23, i32>(55))]
    #[case(new(1000), new(500), new::<12, i64>(1500))]
    #[case(new(-10), new(5), new::<9, i128>(-5))]
    #[case(new(35), new(15), new::<13, i8>(50))]
    #[case(new(250), new(300), new::<28, i16>(550))]
    #[case(new(400), new(100), new::<8, i32>(500))]
    #[case(new(-20), new(20), new::<5, i64>(0))]
    #[case(new(150), new(150), new::<18, i128>(300))]
    #[case(new(3), new(7), new::<2, i8>(10))]
    #[case(new(60), new(40), new::<20, i16>(100))]
    #[case(new(1000), new(-500), new::<17, i32>(500))]
    #[case(new(500), new(200), new::<25, i64>(700))]
    #[case(new(-100), new(50), new::<11, i128>(-50))]
    #[case(new(25), new(30), new::<30, i8>(55))]
    #[case(new(15), new(25), new::<32, i16>(40))]
    #[case(new(8), new(12), new::<21, i32>(20))]
    #[case(new(-25), new(50), new::<9, i64>(25))]
    #[case(new(100), new(-50), new::<12, i128>(50))]
    #[case(new(300), new(200), new::<16, i16>(500))]
    #[case(new(10), new(10), new::<19, i32>(20))]
    #[case(new(-50), new(-50), new::<22, i64>(-100))]
    #[case(new(250), new(100), new::<8, i128>(350))]
    #[case(new(75), new(25), new::<6, i8>(100))]
    #[case(new(150), new(250), new::<5, i16>(400))]
    #[case(new(300), new(100), new::<18, i32>(400))]
    #[case(new(-50), new(75), new::<9, i64>(25))]
    #[case(new(600), new(-300), new::<13, i128>(300))]
    #[case(new(45), new(55), new::<28, i8>(100))]
    #[case(new(30), new(70), new::<2, i16>(100))]
    #[case(new(10), new(200), new::<12, i32>(210))]
    #[case(new(-30), new(-20), new::<11, i64>(-50))]
    #[case(new(10), new(5), new::<30, i128>(15))]
    #[case(new(60), new(40), new::<8, i8>(100))]
    #[case(new(50), new(150), new::<22, i16>(200))]
    #[case(new(200), new(100), new::<20, i32>(300))]
    #[case(new(100), new(-100), new::<18, i64>(0))]
    #[case(new(300), new(100), new::<13, i128>(400))]
    #[case(new(-60), new(10), new::<17, i8>(-50))]
    #[case(new(500), new(100), new::<9, i16>(600))]
    #[case(new(200), new(300), new::<30, i32>(500))]
    #[case(new(10), new(10), new::<15, i64>(20))]
    #[case(new(-100), new(100), new::<8, i128>(0))]
    fn _test_add<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>, 
        #[case] y: Q<A, B, default_engine::DefaultEngine>, 
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>) 
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x + y).expect("");
        assert_eq!(result, correct_result);
    }

    #[rstest]
    #[case(new(10), new(10), new::<1, u8>(0))]
    #[case(new(10), new(10), new::<2, u8>(0))]
    #[case(new(10), new(10), new::<3, u8>(0))]
    #[case(new(10), new(10), new::<4, u8>(0))]
    #[case(new(10), new(10), new::<5, u8>(0))]
    #[case(new(10), new(10), new::<6, u8>(0))]
    #[case(new(10), new(10), new::<7, u8>(0))]
    #[case(new(10), new(10), new::<8, u8>(0))]
    #[case(new(10), new(10), new::<9, u8>(0))]
    #[case(new(10), new(10), new::<10, u8>(0))]
    #[case(new(10), new(10), new::<11, u8>(0))]
    #[case(new(10), new(10), new::<12, u8>(0))]
    #[case(new(10), new(10), new::<13, u8>(0))]
    #[case(new(10), new(10), new::<14, u8>(0))]
    #[case(new(10), new(10), new::<15, u8>(0))]
    #[case(new(10), new(10), new::<16, u8>(0))]
    #[case(new(10), new(10), new::<17, u8>(0))]
    #[case(new(10), new(10), new::<18, u8>(0))]
    #[case(new(10), new(10), new::<19, u8>(0))]
    #[case(new(10), new(10), new::<0, u8>(0))]
    #[case(new(10), new(10), new::<21, u8>(0))]
    #[case(new(10), new(10), new::<22, u8>(0))]
    #[case(new(10), new(10), new::<23, u8>(0))]
    #[case(new(10), new(10), new::<24, u8>(0))]
    #[case(new(10), new(10), new::<25, u8>(0))]
    #[case(new(10), new(10), new::<26, u8>(0))]
    #[case(new(10), new(10), new::<27, u8>(0))]
    #[case(new(10), new(10), new::<28, u8>(0))]
    #[case(new(10), new(10), new::<29, u8>(0))]
    #[case(new(10), new(10), new::<30, u8>(0))]
    #[case(new(10), new(10), new::<31, u8>(0))]
    #[case(new(10), new(10), new::<32, u8>(0))]
    #[case(new(10), new(10), new::<33, u8>(0))]
    #[case(new(10), new(10), new::<34, u8>(0))]
    #[case(new(10), new(10), new::<35, u8>(0))]
    #[case(new(10), new(10), new::<36, u8>(0))]
    #[case(new(10), new(10), new::<37, u8>(0))]
    #[case(new(10), new(10), new::<38, u8>(0))]
    #[case(new(10), new(10), new::<1, u16>(0))]
    #[case(new(10), new(10), new::<2, u16>(0))]
    #[case(new(10), new(10), new::<3, u16>(0))]
    #[case(new(10), new(10), new::<4, u16>(0))]
    #[case(new(10), new(10), new::<5, u16>(0))]
    #[case(new(10), new(10), new::<6, u16>(0))]
    #[case(new(10), new(10), new::<7, u16>(0))]
    #[case(new(10), new(10), new::<8, u16>(0))]
    #[case(new(10), new(10), new::<9, u16>(0))]
    #[case(new(10), new(10), new::<10, u16>(0))]
    #[case(new(10), new(10), new::<11, u16>(0))]
    #[case(new(10), new(10), new::<12, u16>(0))]
    #[case(new(10), new(10), new::<13, u16>(0))]
    #[case(new(10), new(10), new::<14, u16>(0))]
    #[case(new(10), new(10), new::<15, u16>(0))]
    #[case(new(10), new(10), new::<16, u16>(0))]
    #[case(new(10), new(10), new::<17, u16>(0))]
    #[case(new(10), new(10), new::<18, u16>(0))]
    #[case(new(10), new(10), new::<19, u16>(0))]
    #[case(new(10), new(10), new::<0, u16>(0))]
    #[case(new(10), new(10), new::<21, u16>(0))]
    #[case(new(10), new(10), new::<22, u16>(0))]
    #[case(new(10), new(10), new::<23, u16>(0))]
    #[case(new(10), new(10), new::<24, u16>(0))]
    #[case(new(10), new(10), new::<25, u16>(0))]
    #[case(new(10), new(10), new::<26, u16>(0))]
    #[case(new(10), new(10), new::<27, u16>(0))]
    #[case(new(10), new(10), new::<28, u16>(0))]
    #[case(new(10), new(10), new::<29, u16>(0))]
    #[case(new(10), new(10), new::<30, u16>(0))]
    #[case(new(10), new(10), new::<31, u16>(0))]
    #[case(new(10), new(10), new::<32, u16>(0))]
    #[case(new(10), new(10), new::<33, u16>(0))]
    #[case(new(10), new(10), new::<34, u16>(0))]
    #[case(new(10), new(10), new::<35, u16>(0))]
    #[case(new(10), new(10), new::<36, u16>(0))]
    #[case(new(10), new(10), new::<37, u16>(0))]
    #[case(new(10), new(10), new::<38, u16>(0))]
    #[case(new(10), new(10), new::<1, u32>(0))]
    #[case(new(10), new(10), new::<2, u32>(0))]
    #[case(new(10), new(10), new::<3, u32>(0))]
    #[case(new(10), new(10), new::<4, u32>(0))]
    #[case(new(10), new(10), new::<5, u32>(0))]
    #[case(new(10), new(10), new::<6, u32>(0))]
    #[case(new(10), new(10), new::<7, u32>(0))]
    #[case(new(10), new(10), new::<8, u32>(0))]
    #[case(new(10), new(10), new::<9, u32>(0))]
    #[case(new(10), new(10), new::<10, u32>(0))]
    #[case(new(10), new(10), new::<11, u32>(0))]
    #[case(new(10), new(10), new::<12, u32>(0))]
    #[case(new(10), new(10), new::<13, u32>(0))]
    #[case(new(10), new(10), new::<14, u32>(0))]
    #[case(new(10), new(10), new::<15, u32>(0))]
    #[case(new(10), new(10), new::<16, u32>(0))]
    #[case(new(10), new(10), new::<17, u32>(0))]
    #[case(new(10), new(10), new::<18, u32>(0))]
    #[case(new(10), new(10), new::<19, u32>(0))]
    #[case(new(10), new(10), new::<0, u32>(0))]
    #[case(new(10), new(10), new::<21, u32>(0))]
    #[case(new(10), new(10), new::<22, u32>(0))]
    #[case(new(10), new(10), new::<23, u32>(0))]
    #[case(new(10), new(10), new::<24, u32>(0))]
    #[case(new(10), new(10), new::<25, u32>(0))]
    #[case(new(10), new(10), new::<26, u32>(0))]
    #[case(new(10), new(10), new::<27, u32>(0))]
    #[case(new(10), new(10), new::<28, u32>(0))]
    #[case(new(10), new(10), new::<29, u32>(0))]
    #[case(new(10), new(10), new::<30, u32>(0))]
    #[case(new(10), new(10), new::<31, u32>(0))]
    #[case(new(10), new(10), new::<32, u32>(0))]
    #[case(new(10), new(10), new::<33, u32>(0))]
    #[case(new(10), new(10), new::<34, u32>(0))]
    #[case(new(10), new(10), new::<35, u32>(0))]
    #[case(new(10), new(10), new::<36, u32>(0))]
    #[case(new(10), new(10), new::<37, u32>(0))]
    #[case(new(10), new(10), new::<38, u32>(0))]
    #[case(new(10), new(10), new::<1, u64>(0))]
    #[case(new(10), new(10), new::<2, u64>(0))]
    #[case(new(10), new(10), new::<3, u64>(0))]
    #[case(new(10), new(10), new::<4, u64>(0))]
    #[case(new(10), new(10), new::<5, u64>(0))]
    #[case(new(10), new(10), new::<6, u64>(0))]
    #[case(new(10), new(10), new::<7, u64>(0))]
    #[case(new(10), new(10), new::<8, u64>(0))]
    #[case(new(10), new(10), new::<9, u64>(0))]
    #[case(new(10), new(10), new::<10, u64>(0))]
    #[case(new(10), new(10), new::<11, u64>(0))]
    #[case(new(10), new(10), new::<12, u64>(0))]
    #[case(new(10), new(10), new::<13, u64>(0))]
    #[case(new(10), new(10), new::<14, u64>(0))]
    #[case(new(10), new(10), new::<15, u64>(0))]
    #[case(new(10), new(10), new::<16, u64>(0))]
    #[case(new(10), new(10), new::<17, u64>(0))]
    #[case(new(10), new(10), new::<18, u64>(0))]
    #[case(new(10), new(10), new::<19, u64>(0))]
    #[case(new(10), new(10), new::<0, u64>(0))]
    #[case(new(10), new(10), new::<21, u64>(0))]
    #[case(new(10), new(10), new::<22, u64>(0))]
    #[case(new(10), new(10), new::<23, u64>(0))]
    #[case(new(10), new(10), new::<24, u64>(0))]
    #[case(new(10), new(10), new::<25, u64>(0))]
    #[case(new(10), new(10), new::<26, u64>(0))]
    #[case(new(10), new(10), new::<27, u64>(0))]
    #[case(new(10), new(10), new::<28, u64>(0))]
    #[case(new(10), new(10), new::<29, u64>(0))]
    #[case(new(10), new(10), new::<30, u64>(0))]
    #[case(new(10), new(10), new::<31, u64>(0))]
    #[case(new(10), new(10), new::<32, u64>(0))]
    #[case(new(10), new(10), new::<33, u64>(0))]
    #[case(new(10), new(10), new::<34, u64>(0))]
    #[case(new(10), new(10), new::<35, u64>(0))]
    #[case(new(10), new(10), new::<36, u64>(0))]
    #[case(new(10), new(10), new::<37, u64>(0))]
    #[case(new(10), new(10), new::<38, u64>(0))]
    #[case(new(10), new(10), new::<1, u128>(0))]
    #[case(new(10), new(10), new::<2, u128>(0))]
    #[case(new(10), new(10), new::<3, u128>(0))]
    #[case(new(10), new(10), new::<4, u128>(0))]
    #[case(new(10), new(10), new::<5, u128>(0))]
    #[case(new(10), new(10), new::<6, u128>(0))]
    #[case(new(10), new(10), new::<7, u128>(0))]
    #[case(new(10), new(10), new::<8, u128>(0))]
    #[case(new(10), new(10), new::<9, u128>(0))]
    #[case(new(10), new(10), new::<10, u128>(0))]
    #[case(new(10), new(10), new::<11, u128>(0))]
    #[case(new(10), new(10), new::<12, u128>(0))]
    #[case(new(10), new(10), new::<13, u128>(0))]
    #[case(new(10), new(10), new::<14, u128>(0))]
    #[case(new(10), new(10), new::<15, u128>(0))]
    #[case(new(10), new(10), new::<16, u128>(0))]
    #[case(new(10), new(10), new::<17, u128>(0))]
    #[case(new(10), new(10), new::<18, u128>(0))]
    #[case(new(10), new(10), new::<19, u128>(0))]
    #[case(new(10), new(10), new::<0, u128>(0))]
    #[case(new(10), new(10), new::<21, u128>(0))]
    #[case(new(10), new(10), new::<22, u128>(0))]
    #[case(new(10), new(10), new::<23, u128>(0))]
    #[case(new(10), new(10), new::<24, u128>(0))]
    #[case(new(10), new(10), new::<25, u128>(0))]
    #[case(new(10), new(10), new::<26, u128>(0))]
    #[case(new(10), new(10), new::<27, u128>(0))]
    #[case(new(10), new(10), new::<28, u128>(0))]
    #[case(new(10), new(10), new::<29, u128>(0))]
    #[case(new(10), new(10), new::<30, u128>(0))]
    #[case(new(10), new(10), new::<31, u128>(0))]
    #[case(new(10), new(10), new::<32, u128>(0))]
    #[case(new(10), new(10), new::<33, u128>(0))]
    #[case(new(10), new(10), new::<34, u128>(0))]
    #[case(new(10), new(10), new::<35, u128>(0))]
    #[case(new(10), new(10), new::<36, u128>(0))]
    #[case(new(10), new(10), new::<37, u128>(0))]
    #[case(new(10), new(10), new::<38, u128>(0))]
    #[case(new(10), new(10), new::<1, i8>(0))]
    #[case(new(10), new(10), new::<2, i8>(0))]
    #[case(new(10), new(10), new::<3, i8>(0))]
    #[case(new(10), new(10), new::<4, i8>(0))]
    #[case(new(10), new(10), new::<5, i8>(0))]
    #[case(new(10), new(10), new::<6, i8>(0))]
    #[case(new(10), new(10), new::<7, i8>(0))]
    #[case(new(10), new(10), new::<8, i8>(0))]
    #[case(new(10), new(10), new::<9, i8>(0))]
    #[case(new(10), new(10), new::<10, i8>(0))]
    #[case(new(10), new(10), new::<11, i8>(0))]
    #[case(new(10), new(10), new::<12, i8>(0))]
    #[case(new(10), new(10), new::<13, i8>(0))]
    #[case(new(10), new(10), new::<14, i8>(0))]
    #[case(new(10), new(10), new::<15, i8>(0))]
    #[case(new(10), new(10), new::<16, i8>(0))]
    #[case(new(10), new(10), new::<17, i8>(0))]
    #[case(new(10), new(10), new::<18, i8>(0))]
    #[case(new(10), new(10), new::<19, i8>(0))]
    #[case(new(10), new(10), new::<0, i8>(0))]
    #[case(new(10), new(10), new::<21, i8>(0))]
    #[case(new(10), new(10), new::<22, i8>(0))]
    #[case(new(10), new(10), new::<23, i8>(0))]
    #[case(new(10), new(10), new::<24, i8>(0))]
    #[case(new(10), new(10), new::<25, i8>(0))]
    #[case(new(10), new(10), new::<26, i8>(0))]
    #[case(new(10), new(10), new::<27, i8>(0))]
    #[case(new(10), new(10), new::<28, i8>(0))]
    #[case(new(10), new(10), new::<29, i8>(0))]
    #[case(new(10), new(10), new::<30, i8>(0))]
    #[case(new(10), new(10), new::<31, i8>(0))]
    #[case(new(10), new(10), new::<32, i8>(0))]
    #[case(new(10), new(10), new::<33, i8>(0))]
    #[case(new(10), new(10), new::<34, i8>(0))]
    #[case(new(10), new(10), new::<35, i8>(0))]
    #[case(new(10), new(10), new::<36, i8>(0))]
    #[case(new(10), new(10), new::<37, i8>(0))]
    #[case(new(10), new(10), new::<38, i8>(0))]
    #[case(new(10), new(10), new::<1, i16>(0))]
    #[case(new(10), new(10), new::<2, i16>(0))]
    #[case(new(10), new(10), new::<3, i16>(0))]
    #[case(new(10), new(10), new::<4, i16>(0))]
    #[case(new(10), new(10), new::<5, i16>(0))]
    #[case(new(10), new(10), new::<6, i16>(0))]
    #[case(new(10), new(10), new::<7, i16>(0))]
    #[case(new(10), new(10), new::<8, i16>(0))]
    #[case(new(10), new(10), new::<9, i16>(0))]
    #[case(new(10), new(10), new::<10, i16>(0))]
    #[case(new(10), new(10), new::<11, i16>(0))]
    #[case(new(10), new(10), new::<12, i16>(0))]
    #[case(new(10), new(10), new::<13, i16>(0))]
    #[case(new(10), new(10), new::<14, i16>(0))]
    #[case(new(10), new(10), new::<15, i16>(0))]
    #[case(new(10), new(10), new::<16, i16>(0))]
    #[case(new(10), new(10), new::<17, i16>(0))]
    #[case(new(10), new(10), new::<18, i16>(0))]
    #[case(new(10), new(10), new::<19, i16>(0))]
    #[case(new(10), new(10), new::<0, i16>(0))]
    #[case(new(10), new(10), new::<21, i16>(0))]
    #[case(new(10), new(10), new::<22, i16>(0))]
    #[case(new(10), new(10), new::<23, i16>(0))]
    #[case(new(10), new(10), new::<24, i16>(0))]
    #[case(new(10), new(10), new::<25, i16>(0))]
    #[case(new(10), new(10), new::<26, i16>(0))]
    #[case(new(10), new(10), new::<27, i16>(0))]
    #[case(new(10), new(10), new::<28, i16>(0))]
    #[case(new(10), new(10), new::<29, i16>(0))]
    #[case(new(10), new(10), new::<30, i16>(0))]
    #[case(new(10), new(10), new::<31, i16>(0))]
    #[case(new(10), new(10), new::<32, i16>(0))]
    #[case(new(10), new(10), new::<33, i16>(0))]
    #[case(new(10), new(10), new::<34, i16>(0))]
    #[case(new(10), new(10), new::<35, i16>(0))]
    #[case(new(10), new(10), new::<36, i16>(0))]
    #[case(new(10), new(10), new::<37, i16>(0))]
    #[case(new(10), new(10), new::<38, i16>(0))]
    #[case(new(10), new(10), new::<1, i32>(0))]
    #[case(new(10), new(10), new::<2, i32>(0))]
    #[case(new(10), new(10), new::<3, i32>(0))]
    #[case(new(10), new(10), new::<4, i32>(0))]
    #[case(new(10), new(10), new::<5, i32>(0))]
    #[case(new(10), new(10), new::<6, i32>(0))]
    #[case(new(10), new(10), new::<7, i32>(0))]
    #[case(new(10), new(10), new::<8, i32>(0))]
    #[case(new(10), new(10), new::<9, i32>(0))]
    #[case(new(10), new(10), new::<10, i32>(0))]
    #[case(new(10), new(10), new::<11, i32>(0))]
    #[case(new(10), new(10), new::<12, i32>(0))]
    #[case(new(10), new(10), new::<13, i32>(0))]
    #[case(new(10), new(10), new::<14, i32>(0))]
    #[case(new(10), new(10), new::<15, i32>(0))]
    #[case(new(10), new(10), new::<16, i32>(0))]
    #[case(new(10), new(10), new::<17, i32>(0))]
    #[case(new(10), new(10), new::<18, i32>(0))]
    #[case(new(10), new(10), new::<19, i32>(0))]
    #[case(new(10), new(10), new::<0, i32>(0))]
    #[case(new(10), new(10), new::<21, i32>(0))]
    #[case(new(10), new(10), new::<22, i32>(0))]
    #[case(new(10), new(10), new::<23, i32>(0))]
    #[case(new(10), new(10), new::<24, i32>(0))]
    #[case(new(10), new(10), new::<25, i32>(0))]
    #[case(new(10), new(10), new::<26, i32>(0))]
    #[case(new(10), new(10), new::<27, i32>(0))]
    #[case(new(10), new(10), new::<28, i32>(0))]
    #[case(new(10), new(10), new::<29, i32>(0))]
    #[case(new(10), new(10), new::<30, i32>(0))]
    #[case(new(10), new(10), new::<31, i32>(0))]
    #[case(new(10), new(10), new::<32, i32>(0))]
    #[case(new(10), new(10), new::<33, i32>(0))]
    #[case(new(10), new(10), new::<34, i32>(0))]
    #[case(new(10), new(10), new::<35, i32>(0))]
    #[case(new(10), new(10), new::<36, i32>(0))]
    #[case(new(10), new(10), new::<37, i32>(0))]
    #[case(new(10), new(10), new::<38, i32>(0))]
    #[case(new(10), new(10), new::<1, i64>(0))]
    #[case(new(10), new(10), new::<2, i64>(0))]
    #[case(new(10), new(10), new::<3, i64>(0))]
    #[case(new(10), new(10), new::<4, i64>(0))]
    #[case(new(10), new(10), new::<5, i64>(0))]
    #[case(new(10), new(10), new::<6, i64>(0))]
    #[case(new(10), new(10), new::<7, i64>(0))]
    #[case(new(10), new(10), new::<8, i64>(0))]
    #[case(new(10), new(10), new::<9, i64>(0))]
    #[case(new(10), new(10), new::<10, i64>(0))]
    #[case(new(10), new(10), new::<11, i64>(0))]
    #[case(new(10), new(10), new::<12, i64>(0))]
    #[case(new(10), new(10), new::<13, i64>(0))]
    #[case(new(10), new(10), new::<14, i64>(0))]
    #[case(new(10), new(10), new::<15, i64>(0))]
    #[case(new(10), new(10), new::<16, i64>(0))]
    #[case(new(10), new(10), new::<17, i64>(0))]
    #[case(new(10), new(10), new::<18, i64>(0))]
    #[case(new(10), new(10), new::<19, i64>(0))]
    #[case(new(10), new(10), new::<0, i64>(0))]
    #[case(new(10), new(10), new::<21, i64>(0))]
    #[case(new(10), new(10), new::<22, i64>(0))]
    #[case(new(10), new(10), new::<23, i64>(0))]
    #[case(new(10), new(10), new::<24, i64>(0))]
    #[case(new(10), new(10), new::<25, i64>(0))]
    #[case(new(10), new(10), new::<26, i64>(0))]
    #[case(new(10), new(10), new::<27, i64>(0))]
    #[case(new(10), new(10), new::<28, i64>(0))]
    #[case(new(10), new(10), new::<29, i64>(0))]
    #[case(new(10), new(10), new::<30, i64>(0))]
    #[case(new(10), new(10), new::<31, i64>(0))]
    #[case(new(10), new(10), new::<32, i64>(0))]
    #[case(new(10), new(10), new::<33, i64>(0))]
    #[case(new(10), new(10), new::<34, i64>(0))]
    #[case(new(10), new(10), new::<35, i64>(0))]
    #[case(new(10), new(10), new::<36, i64>(0))]
    #[case(new(10), new(10), new::<37, i64>(0))]
    #[case(new(10), new(10), new::<38, i64>(0))]
    #[case(new(10), new(10), new::<1, i128>(0))]
    #[case(new(10), new(10), new::<2, i128>(0))]
    #[case(new(10), new(10), new::<3, i128>(0))]
    #[case(new(10), new(10), new::<4, i128>(0))]
    #[case(new(10), new(10), new::<5, i128>(0))]
    #[case(new(10), new(10), new::<6, i128>(0))]
    #[case(new(10), new(10), new::<7, i128>(0))]
    #[case(new(10), new(10), new::<8, i128>(0))]
    #[case(new(10), new(10), new::<9, i128>(0))]
    #[case(new(10), new(10), new::<10, i128>(0))]
    #[case(new(10), new(10), new::<11, i128>(0))]
    #[case(new(10), new(10), new::<12, i128>(0))]
    #[case(new(10), new(10), new::<13, i128>(0))]
    #[case(new(10), new(10), new::<14, i128>(0))]
    #[case(new(10), new(10), new::<15, i128>(0))]
    #[case(new(10), new(10), new::<16, i128>(0))]
    #[case(new(10), new(10), new::<17, i128>(0))]
    #[case(new(10), new(10), new::<18, i128>(0))]
    #[case(new(10), new(10), new::<19, i128>(0))]
    #[case(new(10), new(10), new::<0, i128>(0))]
    #[case(new(10), new(10), new::<21, i128>(0))]
    #[case(new(10), new(10), new::<22, i128>(0))]
    #[case(new(10), new(10), new::<23, i128>(0))]
    #[case(new(10), new(10), new::<24, i128>(0))]
    #[case(new(10), new(10), new::<25, i128>(0))]
    #[case(new(10), new(10), new::<26, i128>(0))]
    #[case(new(10), new(10), new::<27, i128>(0))]
    #[case(new(10), new(10), new::<28, i128>(0))]
    #[case(new(10), new(10), new::<29, i128>(0))]
    #[case(new(10), new(10), new::<30, i128>(0))]
    #[case(new(10), new(10), new::<31, i128>(0))]
    #[case(new(10), new(10), new::<32, i128>(0))]
    #[case(new(10), new(10), new::<33, i128>(0))]
    #[case(new(10), new(10), new::<34, i128>(0))]
    #[case(new(10), new(10), new::<35, i128>(0))]
    #[case(new(10), new(10), new::<36, i128>(0))]
    #[case(new(10), new(10), new::<37, i128>(0))]
    #[case(new(10), new(10), new::<38, i128>(0))]
    fn _test_sub<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>,
        #[case] y: Q<A, B, default_engine::DefaultEngine>,
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>)
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x - y).unwrap();
        assert_eq!(result, correct_result);
    }

    #[rstest]
    #[case(new(10), new(10), new::<1, u8>(20))]
    #[case(new(10), new(10), new::<2, u8>(20))]
    #[case(new(10), new(10), new::<3, u8>(20))]
    #[case(new(10), new(10), new::<4, u8>(20))]
    #[case(new(10), new(10), new::<5, u8>(20))]
    #[case(new(10), new(10), new::<6, u8>(20))]
    #[case(new(10), new(10), new::<7, u8>(20))]
    #[case(new(10), new(10), new::<8, u8>(20))]
    #[case(new(10), new(10), new::<9, u8>(20))]
    #[case(new(10), new(10), new::<10, u8>(20))]
    #[case(new(10), new(10), new::<11, u8>(20))]
    #[case(new(10), new(10), new::<12, u8>(20))]
    #[case(new(10), new(10), new::<13, u8>(20))]
    #[case(new(10), new(10), new::<14, u8>(20))]
    #[case(new(10), new(10), new::<15, u8>(20))]
    #[case(new(10), new(10), new::<16, u8>(20))]
    #[case(new(10), new(10), new::<17, u8>(20))]
    #[case(new(10), new(10), new::<18, u8>(20))]
    #[case(new(10), new(10), new::<19, u8>(20))]
    #[case(new(10), new(10), new::<20, u8>(20))]
    #[case(new(10), new(10), new::<21, u8>(20))]
    #[case(new(10), new(10), new::<22, u8>(20))]
    #[case(new(10), new(10), new::<23, u8>(20))]
    #[case(new(10), new(10), new::<24, u8>(20))]
    #[case(new(10), new(10), new::<25, u8>(20))]
    #[case(new(10), new(10), new::<26, u8>(20))]
    #[case(new(10), new(10), new::<27, u8>(20))]
    #[case(new(10), new(10), new::<28, u8>(20))]
    #[case(new(10), new(10), new::<29, u8>(20))]
    #[case(new(10), new(10), new::<30, u8>(20))]
    #[case(new(10), new(10), new::<31, u8>(20))]
    #[case(new(10), new(10), new::<32, u8>(20))]
    #[case(new(10), new(10), new::<33, u8>(20))]
    #[case(new(10), new(10), new::<34, u8>(20))]
    #[case(new(10), new(10), new::<35, u8>(20))]
    #[case(new(10), new(10), new::<36, u8>(20))]
    #[case(new(10), new(10), new::<37, u8>(20))]
    #[case(new(10), new(10), new::<38, u8>(20))]
    #[case(new(10), new(10), new::<1, u16>(20))]
    #[case(new(10), new(10), new::<2, u16>(20))]
    #[case(new(10), new(10), new::<3, u16>(20))]
    #[case(new(10), new(10), new::<4, u16>(20))]
    #[case(new(10), new(10), new::<5, u16>(20))]
    #[case(new(10), new(10), new::<6, u16>(20))]
    #[case(new(10), new(10), new::<7, u16>(20))]
    #[case(new(10), new(10), new::<8, u16>(20))]
    #[case(new(10), new(10), new::<9, u16>(20))]
    #[case(new(10), new(10), new::<10, u16>(20))]
    #[case(new(10), new(10), new::<11, u16>(20))]
    #[case(new(10), new(10), new::<12, u16>(20))]
    #[case(new(10), new(10), new::<13, u16>(20))]
    #[case(new(10), new(10), new::<14, u16>(20))]
    #[case(new(10), new(10), new::<15, u16>(20))]
    #[case(new(10), new(10), new::<16, u16>(20))]
    #[case(new(10), new(10), new::<17, u16>(20))]
    #[case(new(10), new(10), new::<18, u16>(20))]
    #[case(new(10), new(10), new::<19, u16>(20))]
    #[case(new(10), new(10), new::<20, u16>(20))]
    #[case(new(10), new(10), new::<21, u16>(20))]
    #[case(new(10), new(10), new::<22, u16>(20))]
    #[case(new(10), new(10), new::<23, u16>(20))]
    #[case(new(10), new(10), new::<24, u16>(20))]
    #[case(new(10), new(10), new::<25, u16>(20))]
    #[case(new(10), new(10), new::<26, u16>(20))]
    #[case(new(10), new(10), new::<27, u16>(20))]
    #[case(new(10), new(10), new::<28, u16>(20))]
    #[case(new(10), new(10), new::<29, u16>(20))]
    #[case(new(10), new(10), new::<30, u16>(20))]
    #[case(new(10), new(10), new::<31, u16>(20))]
    #[case(new(10), new(10), new::<32, u16>(20))]
    #[case(new(10), new(10), new::<33, u16>(20))]
    #[case(new(10), new(10), new::<34, u16>(20))]
    #[case(new(10), new(10), new::<35, u16>(20))]
    #[case(new(10), new(10), new::<36, u16>(20))]
    #[case(new(10), new(10), new::<37, u16>(20))]
    #[case(new(10), new(10), new::<38, u16>(20))]
    #[case(new(10), new(10), new::<1, u32>(20))]
    #[case(new(10), new(10), new::<2, u32>(20))]
    #[case(new(10), new(10), new::<3, u32>(20))]
    #[case(new(10), new(10), new::<4, u32>(20))]
    #[case(new(10), new(10), new::<5, u32>(20))]
    #[case(new(10), new(10), new::<6, u32>(20))]
    #[case(new(10), new(10), new::<7, u32>(20))]
    #[case(new(10), new(10), new::<8, u32>(20))]
    #[case(new(10), new(10), new::<9, u32>(20))]
    #[case(new(10), new(10), new::<10, u32>(20))]
    #[case(new(10), new(10), new::<11, u32>(20))]
    #[case(new(10), new(10), new::<12, u32>(20))]
    #[case(new(10), new(10), new::<13, u32>(20))]
    #[case(new(10), new(10), new::<14, u32>(20))]
    #[case(new(10), new(10), new::<15, u32>(20))]
    #[case(new(10), new(10), new::<16, u32>(20))]
    #[case(new(10), new(10), new::<17, u32>(20))]
    #[case(new(10), new(10), new::<18, u32>(20))]
    #[case(new(10), new(10), new::<19, u32>(20))]
    #[case(new(10), new(10), new::<20, u32>(20))]
    #[case(new(10), new(10), new::<21, u32>(20))]
    #[case(new(10), new(10), new::<22, u32>(20))]
    #[case(new(10), new(10), new::<23, u32>(20))]
    #[case(new(10), new(10), new::<24, u32>(20))]
    #[case(new(10), new(10), new::<25, u32>(20))]
    #[case(new(10), new(10), new::<26, u32>(20))]
    #[case(new(10), new(10), new::<27, u32>(20))]
    #[case(new(10), new(10), new::<28, u32>(20))]
    #[case(new(10), new(10), new::<29, u32>(20))]
    #[case(new(10), new(10), new::<30, u32>(20))]
    #[case(new(10), new(10), new::<31, u32>(20))]
    #[case(new(10), new(10), new::<32, u32>(20))]
    #[case(new(10), new(10), new::<33, u32>(20))]
    #[case(new(10), new(10), new::<34, u32>(20))]
    #[case(new(10), new(10), new::<35, u32>(20))]
    #[case(new(10), new(10), new::<36, u32>(20))]
    #[case(new(10), new(10), new::<37, u32>(20))]
    #[case(new(10), new(10), new::<38, u32>(20))]
    #[case(new(10), new(10), new::<1, u64>(20))]
    #[case(new(10), new(10), new::<2, u64>(20))]
    #[case(new(10), new(10), new::<3, u64>(20))]
    #[case(new(10), new(10), new::<4, u64>(20))]
    #[case(new(10), new(10), new::<5, u64>(20))]
    #[case(new(10), new(10), new::<6, u64>(20))]
    #[case(new(10), new(10), new::<7, u64>(20))]
    #[case(new(10), new(10), new::<8, u64>(20))]
    #[case(new(10), new(10), new::<9, u64>(20))]
    #[case(new(10), new(10), new::<10, u64>(20))]
    #[case(new(10), new(10), new::<11, u64>(20))]
    #[case(new(10), new(10), new::<12, u64>(20))]
    #[case(new(10), new(10), new::<13, u64>(20))]
    #[case(new(10), new(10), new::<14, u64>(20))]
    #[case(new(10), new(10), new::<15, u64>(20))]
    #[case(new(10), new(10), new::<16, u64>(20))]
    #[case(new(10), new(10), new::<17, u64>(20))]
    #[case(new(10), new(10), new::<18, u64>(20))]
    #[case(new(10), new(10), new::<19, u64>(20))]
    #[case(new(10), new(10), new::<20, u64>(20))]
    #[case(new(10), new(10), new::<21, u64>(20))]
    #[case(new(10), new(10), new::<22, u64>(20))]
    #[case(new(10), new(10), new::<23, u64>(20))]
    #[case(new(10), new(10), new::<24, u64>(20))]
    #[case(new(10), new(10), new::<25, u64>(20))]
    #[case(new(10), new(10), new::<26, u64>(20))]
    #[case(new(10), new(10), new::<27, u64>(20))]
    #[case(new(10), new(10), new::<28, u64>(20))]
    #[case(new(10), new(10), new::<29, u64>(20))]
    #[case(new(10), new(10), new::<30, u64>(20))]
    #[case(new(10), new(10), new::<31, u64>(20))]
    #[case(new(10), new(10), new::<32, u64>(20))]
    #[case(new(10), new(10), new::<33, u64>(20))]
    #[case(new(10), new(10), new::<34, u64>(20))]
    #[case(new(10), new(10), new::<35, u64>(20))]
    #[case(new(10), new(10), new::<36, u64>(20))]
    #[case(new(10), new(10), new::<37, u64>(20))]
    #[case(new(10), new(10), new::<38, u64>(20))]
    #[case(new(10), new(10), new::<1, u128>(20))]
    #[case(new(10), new(10), new::<2, u128>(20))]
    #[case(new(10), new(10), new::<3, u128>(20))]
    #[case(new(10), new(10), new::<4, u128>(20))]
    #[case(new(10), new(10), new::<5, u128>(20))]
    #[case(new(10), new(10), new::<6, u128>(20))]
    #[case(new(10), new(10), new::<7, u128>(20))]
    #[case(new(10), new(10), new::<8, u128>(20))]
    #[case(new(10), new(10), new::<9, u128>(20))]
    #[case(new(10), new(10), new::<10, u128>(20))]
    #[case(new(10), new(10), new::<11, u128>(20))]
    #[case(new(10), new(10), new::<12, u128>(20))]
    #[case(new(10), new(10), new::<13, u128>(20))]
    #[case(new(10), new(10), new::<14, u128>(20))]
    #[case(new(10), new(10), new::<15, u128>(20))]
    #[case(new(10), new(10), new::<16, u128>(20))]
    #[case(new(10), new(10), new::<17, u128>(20))]
    #[case(new(10), new(10), new::<18, u128>(20))]
    #[case(new(10), new(10), new::<19, u128>(20))]
    #[case(new(10), new(10), new::<20, u128>(20))]
    #[case(new(10), new(10), new::<21, u128>(20))]
    #[case(new(10), new(10), new::<22, u128>(20))]
    #[case(new(10), new(10), new::<23, u128>(20))]
    #[case(new(10), new(10), new::<24, u128>(20))]
    #[case(new(10), new(10), new::<25, u128>(20))]
    #[case(new(10), new(10), new::<26, u128>(20))]
    #[case(new(10), new(10), new::<27, u128>(20))]
    #[case(new(10), new(10), new::<28, u128>(20))]
    #[case(new(10), new(10), new::<29, u128>(20))]
    #[case(new(10), new(10), new::<30, u128>(20))]
    #[case(new(10), new(10), new::<31, u128>(20))]
    #[case(new(10), new(10), new::<32, u128>(20))]
    #[case(new(10), new(10), new::<33, u128>(20))]
    #[case(new(10), new(10), new::<34, u128>(20))]
    #[case(new(10), new(10), new::<35, u128>(20))]
    #[case(new(10), new(10), new::<36, u128>(20))]
    #[case(new(10), new(10), new::<37, u128>(20))]
    #[case(new(10), new(10), new::<38, u128>(20))]
    #[case(new(10), new(10), new::<1, i8>(20))]
    #[case(new(10), new(10), new::<2, i8>(20))]
    #[case(new(10), new(10), new::<3, i8>(20))]
    #[case(new(10), new(10), new::<4, i8>(20))]
    #[case(new(10), new(10), new::<5, i8>(20))]
    #[case(new(10), new(10), new::<6, i8>(20))]
    #[case(new(10), new(10), new::<7, i8>(20))]
    #[case(new(10), new(10), new::<8, i8>(20))]
    #[case(new(10), new(10), new::<9, i8>(20))]
    #[case(new(10), new(10), new::<10, i8>(20))]
    #[case(new(10), new(10), new::<11, i8>(20))]
    #[case(new(10), new(10), new::<12, i8>(20))]
    #[case(new(10), new(10), new::<13, i8>(20))]
    #[case(new(10), new(10), new::<14, i8>(20))]
    #[case(new(10), new(10), new::<15, i8>(20))]
    #[case(new(10), new(10), new::<16, i8>(20))]
    #[case(new(10), new(10), new::<17, i8>(20))]
    #[case(new(10), new(10), new::<18, i8>(20))]
    #[case(new(10), new(10), new::<19, i8>(20))]
    #[case(new(10), new(10), new::<20, i8>(20))]
    #[case(new(10), new(10), new::<21, i8>(20))]
    #[case(new(10), new(10), new::<22, i8>(20))]
    #[case(new(10), new(10), new::<23, i8>(20))]
    #[case(new(10), new(10), new::<24, i8>(20))]
    #[case(new(10), new(10), new::<25, i8>(20))]
    #[case(new(10), new(10), new::<26, i8>(20))]
    #[case(new(10), new(10), new::<27, i8>(20))]
    #[case(new(10), new(10), new::<28, i8>(20))]
    #[case(new(10), new(10), new::<29, i8>(20))]
    #[case(new(10), new(10), new::<30, i8>(20))]
    #[case(new(10), new(10), new::<31, i8>(20))]
    #[case(new(10), new(10), new::<32, i8>(20))]
    #[case(new(10), new(10), new::<33, i8>(20))]
    #[case(new(10), new(10), new::<34, i8>(20))]
    #[case(new(10), new(10), new::<35, i8>(20))]
    #[case(new(10), new(10), new::<36, i8>(20))]
    #[case(new(10), new(10), new::<37, i8>(20))]
    #[case(new(10), new(10), new::<38, i8>(20))]
    #[case(new(10), new(10), new::<1, i16>(20))]
    #[case(new(10), new(10), new::<2, i16>(20))]
    #[case(new(10), new(10), new::<3, i16>(20))]
    #[case(new(10), new(10), new::<4, i16>(20))]
    #[case(new(10), new(10), new::<5, i16>(20))]
    #[case(new(10), new(10), new::<6, i16>(20))]
    #[case(new(10), new(10), new::<7, i16>(20))]
    #[case(new(10), new(10), new::<8, i16>(20))]
    #[case(new(10), new(10), new::<9, i16>(20))]
    #[case(new(10), new(10), new::<10, i16>(20))]
    #[case(new(10), new(10), new::<11, i16>(20))]
    #[case(new(10), new(10), new::<12, i16>(20))]
    #[case(new(10), new(10), new::<13, i16>(20))]
    #[case(new(10), new(10), new::<14, i16>(20))]
    #[case(new(10), new(10), new::<15, i16>(20))]
    #[case(new(10), new(10), new::<16, i16>(20))]
    #[case(new(10), new(10), new::<17, i16>(20))]
    #[case(new(10), new(10), new::<18, i16>(20))]
    #[case(new(10), new(10), new::<19, i16>(20))]
    #[case(new(10), new(10), new::<20, i16>(20))]
    #[case(new(10), new(10), new::<21, i16>(20))]
    #[case(new(10), new(10), new::<22, i16>(20))]
    #[case(new(10), new(10), new::<23, i16>(20))]
    #[case(new(10), new(10), new::<24, i16>(20))]
    #[case(new(10), new(10), new::<25, i16>(20))]
    #[case(new(10), new(10), new::<26, i16>(20))]
    #[case(new(10), new(10), new::<27, i16>(20))]
    #[case(new(10), new(10), new::<28, i16>(20))]
    #[case(new(10), new(10), new::<29, i16>(20))]
    #[case(new(10), new(10), new::<30, i16>(20))]
    #[case(new(10), new(10), new::<31, i16>(20))]
    #[case(new(10), new(10), new::<32, i16>(20))]
    #[case(new(10), new(10), new::<33, i16>(20))]
    #[case(new(10), new(10), new::<34, i16>(20))]
    #[case(new(10), new(10), new::<35, i16>(20))]
    #[case(new(10), new(10), new::<36, i16>(20))]
    #[case(new(10), new(10), new::<37, i16>(20))]
    #[case(new(10), new(10), new::<38, i16>(20))]
    #[case(new(10), new(10), new::<1, i32>(20))]
    #[case(new(10), new(10), new::<2, i32>(20))]
    #[case(new(10), new(10), new::<3, i32>(20))]
    #[case(new(10), new(10), new::<4, i32>(20))]
    #[case(new(10), new(10), new::<5, i32>(20))]
    #[case(new(10), new(10), new::<6, i32>(20))]
    #[case(new(10), new(10), new::<7, i32>(20))]
    #[case(new(10), new(10), new::<8, i32>(20))]
    #[case(new(10), new(10), new::<9, i32>(20))]
    #[case(new(10), new(10), new::<10, i32>(20))]
    #[case(new(10), new(10), new::<11, i32>(20))]
    #[case(new(10), new(10), new::<12, i32>(20))]
    #[case(new(10), new(10), new::<13, i32>(20))]
    #[case(new(10), new(10), new::<14, i32>(20))]
    #[case(new(10), new(10), new::<15, i32>(20))]
    #[case(new(10), new(10), new::<16, i32>(20))]
    #[case(new(10), new(10), new::<17, i32>(20))]
    #[case(new(10), new(10), new::<18, i32>(20))]
    #[case(new(10), new(10), new::<19, i32>(20))]
    #[case(new(10), new(10), new::<20, i32>(20))]
    #[case(new(10), new(10), new::<21, i32>(20))]
    #[case(new(10), new(10), new::<22, i32>(20))]
    #[case(new(10), new(10), new::<23, i32>(20))]
    #[case(new(10), new(10), new::<24, i32>(20))]
    #[case(new(10), new(10), new::<25, i32>(20))]
    #[case(new(10), new(10), new::<26, i32>(20))]
    #[case(new(10), new(10), new::<27, i32>(20))]
    #[case(new(10), new(10), new::<28, i32>(20))]
    #[case(new(10), new(10), new::<29, i32>(20))]
    #[case(new(10), new(10), new::<30, i32>(20))]
    #[case(new(10), new(10), new::<31, i32>(20))]
    #[case(new(10), new(10), new::<32, i32>(20))]
    #[case(new(10), new(10), new::<33, i32>(20))]
    #[case(new(10), new(10), new::<34, i32>(20))]
    #[case(new(10), new(10), new::<35, i32>(20))]
    #[case(new(10), new(10), new::<36, i32>(20))]
    #[case(new(10), new(10), new::<37, i32>(20))]
    #[case(new(10), new(10), new::<38, i32>(20))]
    #[case(new(10), new(10), new::<1, i64>(20))]
    #[case(new(10), new(10), new::<2, i64>(20))]
    #[case(new(10), new(10), new::<3, i64>(20))]
    #[case(new(10), new(10), new::<4, i64>(20))]
    #[case(new(10), new(10), new::<5, i64>(20))]
    #[case(new(10), new(10), new::<6, i64>(20))]
    #[case(new(10), new(10), new::<7, i64>(20))]
    #[case(new(10), new(10), new::<8, i64>(20))]
    #[case(new(10), new(10), new::<9, i64>(20))]
    #[case(new(10), new(10), new::<10, i64>(20))]
    #[case(new(10), new(10), new::<11, i64>(20))]
    #[case(new(10), new(10), new::<12, i64>(20))]
    #[case(new(10), new(10), new::<13, i64>(20))]
    #[case(new(10), new(10), new::<14, i64>(20))]
    #[case(new(10), new(10), new::<15, i64>(20))]
    #[case(new(10), new(10), new::<16, i64>(20))]
    #[case(new(10), new(10), new::<17, i64>(20))]
    #[case(new(10), new(10), new::<18, i64>(20))]
    #[case(new(10), new(10), new::<19, i64>(20))]
    #[case(new(10), new(10), new::<20, i64>(20))]
    #[case(new(10), new(10), new::<21, i64>(20))]
    #[case(new(10), new(10), new::<22, i64>(20))]
    #[case(new(10), new(10), new::<23, i64>(20))]
    #[case(new(10), new(10), new::<24, i64>(20))]
    #[case(new(10), new(10), new::<25, i64>(20))]
    #[case(new(10), new(10), new::<26, i64>(20))]
    #[case(new(10), new(10), new::<27, i64>(20))]
    #[case(new(10), new(10), new::<28, i64>(20))]
    #[case(new(10), new(10), new::<29, i64>(20))]
    #[case(new(10), new(10), new::<30, i64>(20))]
    #[case(new(10), new(10), new::<31, i64>(20))]
    #[case(new(10), new(10), new::<32, i64>(20))]
    #[case(new(10), new(10), new::<33, i64>(20))]
    #[case(new(10), new(10), new::<34, i64>(20))]
    #[case(new(10), new(10), new::<35, i64>(20))]
    #[case(new(10), new(10), new::<36, i64>(20))]
    #[case(new(10), new(10), new::<37, i64>(20))]
    #[case(new(10), new(10), new::<38, i64>(20))]
    #[case(new(10), new(10), new::<1, i128>(20))]
    #[case(new(10), new(10), new::<2, i128>(20))]
    #[case(new(10), new(10), new::<3, i128>(20))]
    #[case(new(10), new(10), new::<4, i128>(20))]
    #[case(new(10), new(10), new::<5, i128>(20))]
    #[case(new(10), new(10), new::<6, i128>(20))]
    #[case(new(10), new(10), new::<7, i128>(20))]
    #[case(new(10), new(10), new::<8, i128>(20))]
    #[case(new(10), new(10), new::<9, i128>(20))]
    #[case(new(10), new(10), new::<10, i128>(20))]
    #[case(new(10), new(10), new::<11, i128>(20))]
    #[case(new(10), new(10), new::<12, i128>(20))]
    #[case(new(10), new(10), new::<13, i128>(20))]
    #[case(new(10), new(10), new::<14, i128>(20))]
    #[case(new(10), new(10), new::<15, i128>(20))]
    #[case(new(10), new(10), new::<16, i128>(20))]
    #[case(new(10), new(10), new::<17, i128>(20))]
    #[case(new(10), new(10), new::<18, i128>(20))]
    #[case(new(10), new(10), new::<19, i128>(20))]
    #[case(new(10), new(10), new::<20, i128>(20))]
    #[case(new(10), new(10), new::<21, i128>(20))]
    #[case(new(10), new(10), new::<22, i128>(20))]
    #[case(new(10), new(10), new::<23, i128>(20))]
    #[case(new(10), new(10), new::<24, i128>(20))]
    #[case(new(10), new(10), new::<25, i128>(20))]
    #[case(new(10), new(10), new::<26, i128>(20))]
    #[case(new(10), new(10), new::<27, i128>(20))]
    #[case(new(10), new(10), new::<28, i128>(20))]
    #[case(new(10), new(10), new::<29, i128>(20))]
    #[case(new(10), new(10), new::<30, i128>(20))]
    #[case(new(10), new(10), new::<31, i128>(20))]
    #[case(new(10), new(10), new::<32, i128>(20))]
    #[case(new(10), new(10), new::<33, i128>(20))]
    #[case(new(10), new(10), new::<34, i128>(20))]
    #[case(new(10), new(10), new::<35, i128>(20))]
    #[case(new(10), new(10), new::<36, i128>(20))]
    #[case(new(10), new(10), new::<37, i128>(20))]
    #[case(new(10), new(10), new::<38, i128>(20))]
    fn _test_mul<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>,
        #[case] y: Q<A, B, default_engine::DefaultEngine>,
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>)
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x * y).unwrap();
        assert_eq!(result, correct_result);
    }

    #[rstest]
    #[case(new(10), new(10), new::<1, u8>(10))]
    fn _test_div<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>,
        #[case] y: Q<A, B, default_engine::DefaultEngine>,
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>)
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x / y).unwrap();
        assert_eq!(result, correct_result);
    }
}