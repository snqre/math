use crate::math::util_trait::is_int::IsInt;
use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;
use crate::math::util_trait::ink::maybe_has_decode::MaybeHasDecode;
use crate::math::util_trait::ink::maybe_has_encode::MaybeHasEncode;
use crate::math::util_trait::ink::maybe_has_storage_layout::MaybeHasStorageLayout;
use crate::math::util_trait::ink::maybe_has_type_info::MaybeHasTypeInfo;
use crate::math::util::is_compatible_precision::IsCompatiblePrecision;
use crate::math::util::precision::Precision;
use crate::math::component::BitSimulatedMulDivEngine;
use ::core::cmp::Ordering;
use ::core::ops::Add;
use ::core::ops::Sub;
use ::core::ops::Mul;
use ::core::ops::Div;
use ::core::ops::Rem;
use ::num_traits::int::PrimInt as PrimInt;

pub trait IsMulDivEngine: Clone {
    fn algorithm(&self) -> Algorithm;
    fn cast<const A: u8, const B: u8, C: IsInt + HasBrand + HasSignIntrospection, D: IsMulDivEngine>(&self, current: &Q<A, C, D>, algorithm: &Algorithm) -> Result<Q<B, C, D>>
        where
            Precision<A>: IsCompatiblePrecision,
            Precision<B>: IsCompatiblePrecision;
    fn mul<const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: IsMulDivEngine>(&self, data: &Data<A, B, C>) -> Result<Q<A, B, C>> where Precision<A>: IsCompatiblePrecision;
    fn div<const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: IsMulDivEngine>(&self, data: &Data<A, B, C>) -> Result<Q<A, B, C>> where Precision<A>: IsCompatiblePrecision;
}
pub type Algorithm = (U128Algorithm, I128Algorithm);   
pub type U128Algorithm = fn(u128, u128, u128) -> Result<u128>;
pub type I128Algorithm = fn(i128, i128, i128) -> Result<i128>;
#[derive(Debug)]
#[derive(Clone)]
pub struct Data<'a, const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: IsMulDivEngine> where Precision<A>: IsCompatiblePrecision {
    pub x: &'a Q<A, B, C>,
    pub y: &'a Q<A, B, C>,
    pub algorithm: &'a Algorithm
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

pub type Q1<T> = Q<1u8, T, _Engine>;
pub type Q2<T> = Q<2u8, T, _Engine>;
pub type Q3<T> = Q<3u8, T, _Engine>;
pub type Q4<T> = Q<4u8, T, _Engine>;
pub type Q5<T> = Q<5u8, T, _Engine>;
pub type Q6<T> = Q<6u8, T, _Engine>;
pub type Q7<T> = Q<7u8, T, _Engine>;
pub type Q8<T> = Q<8u8, T, _Engine>;
pub type Q9<T> = Q<9u8, T, _Engine>;
pub type Q10<T> = Q<10u8, T, _Engine>;
pub type Q11<T> = Q<11u8, T, _Engine>;
pub type Q12<T> = Q<12u8, T, _Engine>;
pub type Q13<T> = Q<13u8, T, _Engine>;
pub type Q14<T> = Q<14u8, T, _Engine>;
pub type Q15<T> = Q<15u8, T, _Engine>;
pub type Q16<T> = Q<16u8, T, _Engine>;
pub type Q17<T> = Q<17u8, T, _Engine>;
pub type Q18<T> = Q<18u8, T, _Engine>;
pub type Q19<T> = Q<19u8, T, _Engine>;
pub type Q20<T> = Q<20u8, T, _Engine>;
pub type Q21<T> = Q<21u8, T, _Engine>;
pub type Q22<T> = Q<22u8, T, _Engine>;
pub type Q23<T> = Q<23u8, T, _Engine>;
pub type Q24<T> = Q<24u8, T, _Engine>;
pub type Q25<T> = Q<25u8, T, _Engine>;
pub type Q26<T> = Q<26u8, T, _Engine>;
pub type Q27<T> = Q<27u8, T, _Engine>;
pub type Q28<T> = Q<28u8, T, _Engine>;
pub type Q29<T> = Q<29u8, T, _Engine>;
pub type Q30<T> = Q<30u8, T, _Engine>;
pub type Q31<T> = Q<31u8, T, _Engine>;
pub type Q32<T> = Q<32u8, T, _Engine>;
pub type Q33<T> = Q<33u8, T, _Engine>;
pub type Q34<T> = Q<34u8, T, _Engine>;
pub type Q35<T> = Q<35u8, T, _Engine>;
pub type Q36<T> = Q<36u8, T, _Engine>;
pub type Q37<T> = Q<37u8, T, _Engine>;
pub type Q38<T> = Q<38u8, T, _Engine>;
type _Engine = BitSimulatedMulDivEngine::BitSimulatedMulDivEngine;



pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Q: Value cannot be represented.")]
    Overflow,
    #[error("Q: Value cannot be represented.")]
    Underflow,
    #[error("Q: Divisor cannot be zero.")]
    DivByZero,
    #[error("")]
    RemByZero,
    #[error("Q: Cannot safely be converted to the new type.")]
    UnsupportedConversion
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[cfg_attr(feature = "ink", derive(ink::scale::Encode))]
#[cfg_attr(feature = "ink", derive(ink::scale::Decode))]
#[cfg_attr(feature = "ink", derive(ink::scale_info::TypeInfo))]
#[cfg_attr(feature = "ink", derive(ink::storage::traits::StorageLayout))]
pub struct Q<const A: u8, B: IsInt, C: IsMulDivEngine> where Precision<A>: IsCompatiblePrecision {
    pub(super) _v: B,
    pub(super) _engine: C
}

impl<const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: IsMulDivEngine> Rem for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {

    type Output = Result<Self>;

    fn rem(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        if x.is_signed() && y.is_signed() {
            let v_0: i128 = x._v.to_i128().unwrap();
            let v_1: i128 = y._v.to_i128().unwrap();
            if v_1 == 0i128 {
                return Err(Error::DivByZero)
            }
            let v_2: i128 = v_0.checked_rem(v_1).ok_or(Error::RemByZero)?;
            let v_2: B = B::from(v_2).unwrap();
            return Ok(new_with_custom_engine(v_2, x._engine.clone()))   
        }
        debug_assert!(!x.is_signed());
        debug_assert!(!y.is_signed());
        let v_0: u128 = x._v.to_u128().unwrap();
        let v_1: u128 = y._v.to_u128().unwrap();
        if v_1 == 0u128 {
            return Err(Error::DivByZero)
        }
        let v_2: u128 = v_0.checked_rem(v_1).ok_or(Error::RemByZero)?;
        let v_2: B = B::from(v_2).unwrap();
        Ok(new_with_custom_engine(v_2, x._engine.clone()))
    }
}

impl<const A: u8, B: IsInt, C: IsMulDivEngine> Add for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {

    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let engine: C = self._engine.clone();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let value_0: B = x._v;
        let value_1: B = y._v;
        let value_2: B = value_0.checked_add(&value_1).ok_or(Error::Overflow)?;
        Ok(new_with_custom_engine(value_2, engine))
    }
}

impl<const A: u8, B: IsInt, C: IsMulDivEngine> Sub for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {
    
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let engine: C = self._engine.clone();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let value_0: B = x._v;
        let value_1: B = y._v;
        let value_2: B = value_0.checked_sub(&value_1).ok_or(Error::Overflow)?;
        Ok(new_with_custom_engine(value_2, engine))
    }
}

impl<const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: IsMulDivEngine> Mul for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {
    
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: Data<'_, A, B, C> = Data {
            x,
            y,
            algorithm: &self._engine.algorithm()
        };
        self._engine.mul(&data)
    }
}

impl<const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: IsMulDivEngine> Div for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {
    
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: Data<'_, A, B, C> = Data {
            x,
            y,
            algorithm: &self._engine.algorithm()
        };
        self._engine.div(&data)   
    }
}

impl<const A: u8, B: IsInt + HasSignIntrospection, C: IsMulDivEngine> Q<A, B, C> where Precision<A>: IsCompatiblePrecision {
    
    pub fn max_underlying_value(&self) -> B {
        B::max_value()
    }

    pub fn min_underlying_value(&self) -> B {
        B::min_value()
    }
}

impl<const A: u8, B: IsInt, C: IsMulDivEngine> PartialOrd for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {

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

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const A: u8, B: IsInt, C: IsMulDivEngine> Ord for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {

    fn clamp(self, min: Self, max: Self) -> Self {
        if self > max {
            return max
        }
        if self < min {
            return min
        }
        self
    }

    fn max(self, other: Self) -> Self {
        let x: &Self = &self;
        let y: &Self = &other;
        let v_0: B = x._v;
        let v_1: B = y._v;
        let v_2: B = v_0.max(v_1);
        if v_2 == v_0 {
            return new_with_custom_engine(v_0, x._engine.clone())
        } 
        new_with_custom_engine(v_1, x._engine.clone())
    }

    fn min(self, other: Self) -> Self {
        let x: &Self = &self;
        let y: &Self = &other;
        let v_0: B = x._v;
        let v_1: B = y._v;
        let v_2: B = v_0.min(v_1);
        if v_2 == v_0 {
            return new_with_custom_engine(v_0, x._engine.clone())
        }
        new_with_custom_engine(v_1, x._engine.clone())
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let x: &Self = self;
        let y: &Self = other;
        let v_0: B = x._v;
        let v_1: B = y._v;
        v_0.cmp(&v_1)
    }
}

impl<const A: u8, B: IsInt, C: IsMulDivEngine> PartialEq for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {

    fn eq(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        v_0 == v_1
    }
}

impl<const A: u8, B: IsInt, C: IsMulDivEngine> Eq for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {}

impl<const A: u8, B: IsInt + HasSignIntrospection, C: IsMulDivEngine> HasSignIntrospection for Q<A, B, C> where Precision<A>: IsCompatiblePrecision {

    fn is_signed(&self) -> bool {
        self._v.is_signed()
    }
}

impl<const A: u8, B: IsInt + HasSignIntrospection, C: IsMulDivEngine> Q<A, B, C> where Precision<A>: IsCompatiblePrecision {

    pub fn to_u128(&self) -> Result<u128> {
        self._v.to_u128().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_u64(&self) -> Result<u64> {
        self._v.to_u64().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_u32(&self) -> Result<u32> {
        self._v.to_u32().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_u16(&self) -> Result<u16> {
        self._v.to_u16().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_u8(&self) -> Result<u8> {
        self._v.to_u8().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_i128(&self) -> Result<i128> {
        self._v.to_i128().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_i64(&self) -> Result<i64> {
        self._v.to_i64().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_i32(&self) -> Result<i32> {
        self._v.to_i32().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_i16(&self) -> Result<i16> {
        self._v.to_i16().ok_or(Error::UnsupportedConversion)
    }

    pub fn to_i8(&self) -> Result<i8> {
        self._v.to_i8().ok_or(Error::UnsupportedConversion)
    }

}

pub fn new<const A: u8, B: IsInt>(v: B) -> Q<A, B, _Engine> where Precision<A>: IsCompatiblePrecision {
    debug_assert!(matches!(A, 1u8..=38u8));
    Q {
        _v: v,
        _engine: BitSimulatedMulDivEngine::BitSimulatedMulDivEngine
    }
}

pub fn new_with_custom_engine<const A: u8, B: IsInt, C: IsMulDivEngine>(value: B, engine: C) -> Q<A, B, C> where Precision<A>: IsCompatiblePrecision {
    debug_assert!(matches!(A, 1u8..=38u8));
    Q {
        _v: value,
        _engine: engine
    }
}

pub fn new_from_integer<
    const A: u8, 
    const B: u8, 
    C: IsInt + HasBrand + HasSignIntrospection, 
    D: IsInt + HasBrand + HasSignIntrospection>(value: C) -> Result<Q<B, D, _Engine>> 
    where
        Precision<A>: IsCompatiblePrecision,
        Precision<B>: IsCompatiblePrecision {
    debug_assert!(matches!(A, 1u8..=38u8));
    debug_assert!(matches!(B, 1u8..=38u8));
    let value: D = D::from(value).ok_or(Error::UnsupportedConversion)?;
    let value: Q<A, D, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine> = new(value);
    let engine: _Engine = BitSimulatedMulDivEngine::default();
    let algorithm: Algorithm = engine.algorithm();
    engine.cast(&value, &algorithm)
}
 
pub fn new_from_integer_with_custom_engine<
    const A: u8, 
    const B: u8, 
    C: IsInt + HasBrand + HasSignIntrospection,
    D: IsInt + HasBrand + HasSignIntrospection, 
    E: IsMulDivEngine>(value: C, engine: E) -> Result<Q<B, D, E>> 
    where
        Precision<A>: IsCompatiblePrecision,
        Precision<B>: IsCompatiblePrecision {
    debug_assert!(matches!(A, 1u8..=38u8));
    debug_assert!(matches!(B, 1u8..=38u8));
    let new_engine: E = engine.clone();
    let value: D = D::from(value).ok_or(Error::UnsupportedConversion)?;
    let value: Q<A, D, E> = new_with_custom_engine(value, new_engine);
    let algorithm: Algorithm = engine.algorithm();
    engine.cast(&value, &algorithm)
}