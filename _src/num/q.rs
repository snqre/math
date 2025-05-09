use crate::num::int_i::*;
use crate::num::precision::*;

pub trait Engine: Sized + Clone + Copy {

    fn sqrt<const A: u8, B>(&self, x: &Q<A, B, Self>) -> Result<Q<A, B, Self>> where B: IntIntrospectionI, Precision<A>: PrecisionCompatibleI {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let scale: u128 = Precision::<A>::u128_scale();
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

    fn cast<const A: u8, const B: u8, C: HasIntrospection>(&self, x: &Q<A, C, Self>) -> Result<Q<B, C, Self>> 
    where
        Precision<A>: IsValidPrecision,
        Precision<B>: IsValidPrecision {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: C = x._v;
        if A == B {
            let result: C = x;
            let result: Q<B, C, Self> = new_with_custom_engine(&result, self);
            return Ok(result)
        }
        if x.is_signed() {
            let old_scale: i128 = Precision::<A>::i128_scale();
            let new_scale: i128 = Precision::<B>::i128_scale();
            let result: i128 = x.to_i128().unwrap();
            let result: i128 = self.muldiv(&result, &new_scale, &old_scale)?;
            let result: C = C::from(result).unwrap();
            let result: Q<B, C, Self> = new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let old_scale: u128 = Precision::<A>::u128_scale();
        let new_scale: u128 = Precision::<B>::u128_scale();
        let result: u128 = x.to_u128().unwrap();
        let result: u128 = self.muldiv(&result, &new_scale, &old_scale)?;
        let result: C = C::from(result).unwrap();
        let result: Q<B, C, Self> = new_with_custom_engine(&result, self);
        Ok(result)
    }

    fn mul<const A: u8, B>(
        &self, 
        x: &Q<A, B, Self>, 
        y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where     
        B: int::Int,
        B: int_introspection::Introspection,
        precision::Precision<A>: precision::IsValidPrecision {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
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
        let result: Q<A, B, Self> = new_with_custom_engine(&result, self);
        Ok(result)
    }

    fn div<const A: u8, B>(
        &self, 
        x: &Q<A, B, Self>, 
        y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where 
        B: int::Int,
        B: int_introspection::Introspection,
        precision::Precision<A>: precision::IsValidPrecision {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: &B = &x._v;
        let y: &B = &y._v;
        let scale: u128 = precision::u128_scale::<A>();
        if x.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            if scale.is_power_of_two() {
                let z: u32 = scale.trailing_zeros();
                let result: i128 = (x << z).checked_div(y).ok_or(Error::DivisionByZero)?;
                let result: &B = &B::from(result).unwrap();
                let result: Q<A, B, Self> = new_with_custom_engine(result, self);
                return Ok(result)
            }
            let scale: i128 = precision::i128_scale::<A>();
            let result: i128 = self.muldiv(&x, &scale, &y)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        if scale.is_power_of_two() {
            let z: u32 = scale.trailing_zeros();
            let result: u128 = (x << z).checked_div(y).ok_or(Error::DivisionByZero)?;
            let result: B = B::from(result).unwrap();
            let result: Q<A, B, Self> = new_with_custom_engine(&result, self);
            return Ok(result)
        }
        let result: u128 = self.muldiv(&x, &scale, &y)?;
        let result: B = B::from(result).unwrap();
        let result: Q<A, B, Self> = new_with_custom_engine(&result, self);
        Ok(result)
    }

    fn muldiv<'a, T>(
        &self, 
        x: &'a T, 
        y: &'a T, 
        z: &'a T) -> Result<T> 
    where 
        T: int::Int,
        T: int_introspection::Introspection {
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
                        .to_int()?;
                    return Ok(result)
                }
                let result: T = result.to_int()?;
                return Ok(result)
            }
            let result: u128 = _fold_128_bit_product_mod(a, b, z) / z;
            if sign == 1 {
                let result: T = result
                    .to_i128()
                    .ok_or(Error::UnsupportedConversion)?
                    .wrapping_neg()
                    .to_int()?;
                return Ok(result)
            }
            let result: T = result.to_int()?;
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
            let result: T = result.to_int()?;
            return Ok(result);
        }
        let result: u128 = _fold_128_bit_product_mod(a, b, z) / z;
        let result: T = result.to_int()?;
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

/// <h1>Range</h1>
/// <p1>0..~25</p1>
pub type Q1U8 = Q1<u8>;

/// `0` -> `2.55E+00` `~2`
pub type Q2U8 = Q2<u8>;


/// `0` -> `2.55E+03` `K ~6`
pub type Q1U16 = Q1<u16>;

/// `0` -> `2.55E+02` `~655`
pub type Q2U16 = Q2<u16>;

/// `0` -> `2.55E+01` `~64`
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
pub type Q10U64 = Q10<u64>;
pub type Q11U64 = Q11<u64>;
pub type Q12U64 = Q12<u64>;
pub type Q13U64 = Q13<u64>;
pub type Q14U64 = Q14<u64>;
pub type Q15U64 = Q15<u64>;
pub type Q16U64 = Q16<u64>;
pub type Q17U64 = Q17<u64>;
pub type Q18U64 = Q18<u64>;
pub type Q19U64 = Q19<u64>;

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
#[derive(::thiserror::Error)]
pub enum Error {
    #[error("")]
    IntIntrospection(#[from] IntrospectionError),
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
pub struct Q<const A: u8, B: IntIntrospectionI, C: EngineI> where Precision<A>: PrecisionCompatibleI {
    _v: B,
    _engine: C
}










pub fn new<const A: u8, B>(value: B) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int_introspection::Introspection,
    precision::Precision<A>: precision::IsValidPrecision {
    Q {
        _v: value,
        _engine: default_engine::new()
    }
}

pub fn new_with_custom_engine<const A: u8, B, C>(value: &B, engine: &C) -> Q<A, B, C>
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine,
    precision::Precision<A>: precision::IsValidPrecision {
    Q { _v: *value, _engine: engine.clone() }
}

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {
    
    pub fn sqrt(&self) -> Result<Self> {
        self._engine.sqrt(self)
    }
}

impl<const A: u8, B, C> ops::Add for Q<A, B, C> 
    where 
        B: int::Int,
        C: IsQEngine, 
        precision::Precision<A>: precision::IsValidPrecision {
 
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
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
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {

    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: &Self = &self;
        let x: &B = &x._v;
        let y: &Self = &rhs;
        let y: &B = &y._v;
        let result: B = x.checked_sub(y).ok_or(Error::Overflow)?;
        Ok(new_with_custom_engine(&result, &self._engine))
    }
}

impl<const A: u8, B, C> ops::Mul for Q<A, B, C> 
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {

    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.mul(x, y)
    }
}

impl<const A: u8, B, C> ops::Div for Q<A, B, C>
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {
    
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.div(x, y)
    }
}

impl<const A: u8, B, C> Ord for Q<A, B, C> 
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {

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

impl<const A: u8, B, C> PartialOrd for Q<A, B, C> 
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {

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

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {

    fn eq(&self, other: &Self) -> bool {
        let x: &Self = self;
        let x: &B = &x._v;
        let y: &Self = other;
        let y: &B = &y._v;
        x == y
    }
}

impl<const A: u8, B, C> Eq for Q<A, B, C> 
where
    B: int::Int,
    B: int_introspection::Introspection,
    C: IsQEngine, 
    precision::Precision<A>: precision::IsValidPrecision {}