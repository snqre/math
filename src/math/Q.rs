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
use ::core::ops::Add;
use ::core::ops::Sub;
use ::core::ops::Mul;
use ::core::ops::Div;
use ::num_traits::int::PrimInt as PrimInt;

pub trait IsMulDivEngine
    where
        Self: Clone {

    fn algorithm(&self) -> Algorithm;
    fn cast<const T_OLD_PRECISION: u8, const T_NEW_PRECISION: u8, TInteger, TEngine>(&self, current: &Q<T_OLD_PRECISION, TInteger, TEngine>, algo: &Algorithm) -> Result<Q<T_NEW_PRECISION, TInteger, TEngine>>
        where
            TInteger: PrimInt,
            TInteger: HasBrand,
            TInteger: HasSignIntrospection,
            TInteger: MaybeHasDecode,
            TInteger: MaybeHasEncode,
            TInteger: MaybeHasStorageLayout,
            TInteger: MaybeHasTypeInfo,
            TEngine: IsMulDivEngine,
            Precision<T_OLD_PRECISION>: IsCompatiblePrecision,
            Precision<T_NEW_PRECISION>: IsCompatiblePrecision;
    fn mul<const T_PRECISION: u8, TInteger, TEngine>(&self, data: &Data<'_, T_PRECISION, TInteger, TEngine>) -> Result<Q<T_PRECISION, TInteger, TEngine>>
        where
            TInteger: PrimInt,
            TInteger: HasBrand,
            TInteger: HasSignIntrospection,
            TInteger: MaybeHasDecode,
            TInteger: MaybeHasEncode,
            TInteger: MaybeHasStorageLayout,
            TInteger: MaybeHasTypeInfo,
            TEngine: IsMulDivEngine,
            Precision<T_PRECISION>: IsCompatiblePrecision;
    fn div<const T_PRECISION: u8, TInteger, TEngine>(&self, data: &Data<'_, T_PRECISION, TInteger, TEngine>) -> Result<Q<T_PRECISION, TInteger, TEngine>>
        where
            TInteger: PrimInt,
            TInteger: HasBrand,
            TInteger: HasSignIntrospection,
            TInteger: MaybeHasDecode,
            TInteger: MaybeHasEncode,
            TInteger: MaybeHasStorageLayout,
            TInteger: MaybeHasTypeInfo,
            TEngine: IsMulDivEngine,
            Precision<T_PRECISION>: IsCompatiblePrecision;
}
pub type Algorithm = (U128Algorithm, I128Algorithm);   
pub type U128Algorithm = fn(u128, u128, u128) -> Result<u128>;
pub type I128Algorithm = fn(i128, i128, i128) -> Result<i128>;
#[derive(Debug)]
#[derive(Clone)]
pub struct Data<'l1, const T_PRECISION: u8, TInteger, TEngine> 
    where
        TInteger: PrimInt,
        TInteger: HasBrand,
        TInteger: HasSignIntrospection,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_PRECISION>: IsCompatiblePrecision {
    pub x: &'l1 Q<T_PRECISION, TInteger, TEngine>,
    pub y: &'l1 Q<T_PRECISION, TInteger, TEngine>,
    pub algorithm: &'l1 Algorithm
}

pub type Q1<TInteger> = Q<1u8, TInteger, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine>;
pub type Q2<TInteger> = Q<2u8, TInteger, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine>;
pub type Q3<TInteger> = Q<3u8, TInteger, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine>;
pub type Q4<TInteger> = Q<4u8, TInteger, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine>;

pub type Result<TOk> = core::result::Result<TOk, Error>;
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
    DivisionByZero,
    #[error("Q: Cannot safely be converted to the new type.")]
    UnsupportedConversion,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[cfg_attr(feature = "ink", derive(ink::scale::Encode))]
#[cfg_attr(feature = "ink", derive(ink::scale::Decode))]
#[cfg_attr(feature = "ink", derive(ink::scale_info::TypeInfo))]
#[cfg_attr(feature = "ink", derive(ink::storage::traits::StorageLayout))]
pub struct Q<const T_PRECISION: u8, TInteger, TEngine> 
    where 
        TInteger: PrimInt,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_PRECISION>: IsCompatiblePrecision {
    pub(crate) _v: TInteger,
    pub(crate) _engine: TEngine
}

impl<const T_PRECISION: u8, TInteger, TEngine> Add for Q<T_PRECISION, TInteger, TEngine> 
    where 
        TInteger: PrimInt,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_PRECISION>: IsCompatiblePrecision {

    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        use Error::*;
        let engine: TEngine = self._engine.clone();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: TInteger = x._v;
        let v_1: TInteger = y._v;
        let v_2: TInteger = v_0.checked_add(&v_1).ok_or(Overflow)?;
        let v_2: Self = new_with_custom_engine(v_2, engine);
        Ok(v_2)
    }
}

impl<const T_PRECISION: u8, TInteger, TEngine> Sub for Q<T_PRECISION, TInteger, TEngine>
    where 
        TInteger: PrimInt,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_PRECISION>: IsCompatiblePrecision {
    
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        use Error::*;
        let engine: TEngine = self._engine.clone();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: TInteger = x._v;
        let v_1: TInteger = y._v;
        let v_2: TInteger = v_0.checked_sub(&v_1).ok_or(Overflow)?;
        let v_2: Self = new_with_custom_engine(v_2, engine);
        Ok(v_2)
    }
}

impl<const T_PRECISION: u8, TInteger, TEngine> Mul for Q<T_PRECISION, TInteger, TEngine> 
    where
        TInteger: PrimInt,
        TInteger: HasBrand,
        TInteger: HasSignIntrospection,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_PRECISION>: IsCompatiblePrecision {
    
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: Data<'_, T_PRECISION, TInteger, TEngine> = Data {
            x,
            y,
            algorithm: &self._engine.algorithm()
        };
        self._engine.mul(&data)
    }
}

impl<const T_PRECISION: u8, TInteger, TEngine> Div for Q<T_PRECISION, TInteger, TEngine>
    where
        TInteger: PrimInt,
        TInteger: HasBrand,
        TInteger: HasSignIntrospection,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_PRECISION>: IsCompatiblePrecision {
    
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: Data<'_, T_PRECISION, TInteger, TEngine> = Data {
            x,
            y,
            algorithm: &self._engine.algorithm()
        };
        self._engine.div(&data)   
    }
}

impl<const A: u8, B, C> Q<A, B, C> 
    where
        B: PrimInt,
        B: HasSignIntrospection,
        B: MaybeHasDecode,
        B: MaybeHasEncode,
        B: MaybeHasStorageLayout,
        B: MaybeHasTypeInfo,
        C: IsMulDivEngine,
        Precision<A>: IsCompatiblePrecision {
    
    pub fn max_underlying_value(&self) -> B {
        B::max_value()
    }

    pub fn min_underlying_value(&self) -> B {
        B::min_value()
    }
}

impl<const A: u8, B, C> HasSignIntrospection for Q<A, B, C> 
    where
        B: PrimInt,
        B: HasSignIntrospection,
        B: MaybeHasDecode,
        B: MaybeHasEncode,
        B: MaybeHasStorageLayout,
        B: MaybeHasTypeInfo,
        C: IsMulDivEngine,
        Precision<A>: IsCompatiblePrecision {

    fn is_signed(&self) -> bool {
        self._v.is_signed()
    }
}

impl<const A: u8, B, C> Q<A, B, C>
    where
        B: PrimInt,
        B: HasSignIntrospection,
        B: MaybeHasDecode,
        B: MaybeHasEncode,
        B: MaybeHasStorageLayout,
        B: MaybeHasTypeInfo,
        C: IsMulDivEngine,
        Precision<A>: IsCompatiblePrecision {

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

pub fn new<const T_PRECISION: u8, TInteger>(v: TInteger) -> Q<T_PRECISION, TInteger, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine> 
    where 
        TInteger: PrimInt,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        Precision<T_PRECISION>: IsCompatiblePrecision {
    debug_assert!(matches!(T_PRECISION, 1u8..=38u8));
    Q {
        _v: v,
        _engine: BitSimulatedMulDivEngine::BitSimulatedMulDivEngine
    }
}

pub fn new_with_custom_engine<const T_PRECISION: u8, TInteger, TEngine>(v: TInteger, engine: TEngine) -> Q<T_PRECISION, TInteger, TEngine>
    where
        TInteger: PrimInt,
        TInteger: MaybeHasDecode,
        TInteger: MaybeHasEncode,
        TInteger: MaybeHasStorageLayout,
        TInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_PRECISION>: IsCompatiblePrecision {
    debug_assert!(matches!(T_PRECISION, 1u8..=38u8));
    Q {
        _v: v,
        _engine: engine
    }
}

pub fn new_from_integer<const T_OLD_PRECISION: u8, const T_NEW_PRECISION: u8, TOldInteger, TNewInteger>(value: TOldInteger) -> Result<Q<T_NEW_PRECISION, TNewInteger, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine>> 
    where
        TOldInteger: PrimInt,
        TOldInteger: HasBrand,
        TOldInteger: HasSignIntrospection,
        TOldInteger: MaybeHasDecode,
        TOldInteger: MaybeHasEncode,
        TOldInteger: MaybeHasStorageLayout,
        TOldInteger: MaybeHasTypeInfo,
        TNewInteger: PrimInt,
        TNewInteger: HasBrand,
        TNewInteger: HasSignIntrospection,
        TNewInteger: MaybeHasDecode,
        TNewInteger: MaybeHasEncode,
        TNewInteger: MaybeHasStorageLayout,
        TNewInteger: MaybeHasTypeInfo,
        Precision<T_OLD_PRECISION>: IsCompatiblePrecision,
        Precision<T_NEW_PRECISION>: IsCompatiblePrecision {
    let value: TNewInteger = TNewInteger::from(value).ok_or(Error::UnsupportedConversion)?;
    let value: Q<T_OLD_PRECISION, TNewInteger, BitSimulatedMulDivEngine::BitSimulatedMulDivEngine> = new(value);
    let engine: BitSimulatedMulDivEngine::BitSimulatedMulDivEngine = BitSimulatedMulDivEngine::default();
    let algorithm: Algorithm = engine.algorithm();
    engine.cast(&value, &algorithm)
}

pub fn new_from_integer_with_custom_engine<const T_OLD_PRECISION: u8, const T_NEW_PRECISION: u8, TOldInteger, TNewInteger, TEngine>(value: TOldInteger, engine: TEngine) -> Result<Q<T_NEW_PRECISION, TNewInteger, TEngine>> 
    where
        TOldInteger: PrimInt,
        TOldInteger: HasBrand,
        TOldInteger: HasSignIntrospection,
        TOldInteger: MaybeHasDecode,
        TOldInteger: MaybeHasEncode,
        TOldInteger: MaybeHasStorageLayout,
        TOldInteger: MaybeHasTypeInfo,
        TNewInteger: PrimInt,
        TNewInteger: HasBrand,
        TNewInteger: HasSignIntrospection,
        TNewInteger: MaybeHasDecode,
        TNewInteger: MaybeHasEncode,
        TNewInteger: MaybeHasStorageLayout,
        TNewInteger: MaybeHasTypeInfo,
        TEngine: IsMulDivEngine,
        Precision<T_OLD_PRECISION>: IsCompatiblePrecision,
        Precision<T_NEW_PRECISION>: IsCompatiblePrecision {
    let engine_0: TEngine = engine.clone();
    let value: TNewInteger = TNewInteger::from(value).ok_or(Error::UnsupportedConversion)?;
    let value: Q<T_OLD_PRECISION, TNewInteger, TEngine> = new_with_custom_engine(value, engine_0);
    let algorithm: Algorithm = engine.algorithm();
    engine.cast(&value, &algorithm)
}