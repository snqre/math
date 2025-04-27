use crate::math::mul_div_engine::*;
use crate::math::ink::tr_maybe_decode::MaybeDecodeTrait;
use crate::math::ink::tr_maybe_encode::MaybeEncodeTrait;
use crate::math::ink::tr_maybe_storage_layout::MaybeStorageLayoutTrait;
use crate::math::ink::tr_maybe_type_info::MaybeTypeInfoTrait;
use crate::math::sign_introspection_trait::SignIntrospectionTrait;
use crate::math::precision::Precision;
use crate::math::precision_trait::PrecisionTrait;
use crate::math::brandable_t::BrandableT;
use crate::math::q_trait::QTraitError;
use crate::math::q_trait::QTraitResult;
use core::fmt::Debug as DebugTrait;
use core::clone::Clone as CloneTrait;
use core::marker::Copy as CopyTrait;
use core::ops::Add as AddTrait;
use core::ops::Sub as SubTrait;
use core::ops::Mul as MulTrait;
use core::ops::Div as DivTrait;
use core::ops::Rem as RemTrait;
use core::cmp::PartialEq as PartialEqTrait;
use core::cmp::Eq as EqTrait;
use core::cmp::PartialOrd as PartialOrdTrait;
use core::cmp::Ord as OrdTrait;
use core::cmp::Ordering;
use num_traits::int::PrimInt as PrimIntTrait;
#[cfg(not(feature = "ink"))]
use num_traits::float::Float as FloatTrait;

#[cfg(feature = "ink")]
use ink::scale::Encode as EncodeTrait;
#[cfg(feature = "ink")]
use ink::scale::Decode as DecodeTrait;
#[cfg(feature = "ink")]
use ink::scale_info::TypeInfo as TypeInfoTrait;
#[cfg(feature = "ink")]
use ink::storage::traits::StorageLayout as StorageLayoutTrait;

macro_rules! q_generic_variant {
    ($name:ident, $size:expr) => {
        #[allow(type_alias_bounds)]
        pub type $name<
            T: PrimIntTrait
            + MaybeEncodeTrait
            + MaybeDecodeTrait
            + MaybeTypeInfoTrait
            + MaybeStorageLayoutTrait
        > = Q<$size, T>;
    };
}

q_generic_variant!(Q1, 1u8);
q_generic_variant!(Q2, 2u8);
q_generic_variant!(Q3, 3u8);
q_generic_variant!(Q4, 4u8);
q_generic_variant!(Q5, 5u8);
q_generic_variant!(Q6, 6u8);
q_generic_variant!(Q7, 7u8);
q_generic_variant!(Q8, 8u8);
q_generic_variant!(Q9, 9u8);
q_generic_variant!(Q10, 10u8);
q_generic_variant!(Q11, 11u8);
q_generic_variant!(Q12, 12u8);
q_generic_variant!(Q13, 13u8);
q_generic_variant!(Q14, 14u8);
q_generic_variant!(Q15, 15u8);
q_generic_variant!(Q16, 16u8);
q_generic_variant!(Q17, 17u8);
q_generic_variant!(Q18, 18u8);
q_generic_variant!(Q19, 19u8);
q_generic_variant!(Q20, 20u8);
q_generic_variant!(Q21, 21u8);
q_generic_variant!(Q22, 22u8);
q_generic_variant!(Q23, 23u8);
q_generic_variant!(Q24, 24u8);
q_generic_variant!(Q25, 25u8);
q_generic_variant!(Q26, 26u8);
q_generic_variant!(Q27, 27u8);
q_generic_variant!(Q28, 28u8);
q_generic_variant!(Q29, 29u8);
q_generic_variant!(Q30, 30u8);
q_generic_variant!(Q31, 31u8);
q_generic_variant!(Q32, 32u8);
q_generic_variant!(Q33, 33u8);
q_generic_variant!(Q34, 34u8);
q_generic_variant!(Q35, 35u8);

pub type Q1U8 = Q1<u8>;

#[derive(DebugTrait)]
#[derive(CloneTrait)]
#[derive(CopyTrait)]
#[cfg_attr(feature = "ink", derive(EncodeTrait))]
#[cfg_attr(feature = "ink", derive(DecodeTrait))]
#[cfg_attr(feature = "ink", derive(TypeInfoTrait))]
#[cfg_attr(feature = "ink", derive(StorageLayoutTrait))]
pub struct Q<const A: u8, B> 
    where 
        B: PrimIntTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {
    pub(super) _v: B,
}

#[cfg(not(feature = "ink"))]
impl<const A: u8, B> Q<A, B> where 
    B: PrimIntTrait,
    Precision<A>: PrecisionTrait {

    pub fn new_from_float<const C: u8, D: FloatTrait>(v: D) -> QTraitResult<Self> where Precision<C>: PrecisionTrait, {
        let decimals: u32 = A.into();
        let scale = D::from(10u32).ok_or(QTraitError::ConversionFailure)?.powi(decimals);
        let v: D = v.checked_mul(scale).ok_or(QTraitError::Overflow)?;
        if B::zero().is_signed() {
            let v: i128 = v.to_i128().ok_or(QTraitError::ConversionFailure)?;
            if v > B::max_value().to_i128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            if v < B::min_value().to_i128().unwrap() {
                return Err(QTraitError::Underflow)
            }
            let v: B = B::from(v).ok_or(QTraitError::ConversionFailure)?;
            Ok(q(v))
        }
        let v: u128 = v.to_u128().ok_or(QTraitError::ConversionFailure)?;
        if v > B::max_value().to_u128().unwrap() {
            return Err(QTraitError::Overflow)
        }
        let v: B = B::from(v).ok_or(QTraitError::ConversionFailure)?;
        Ok(q(v))
    }
}

impl<const T1: u8, T2> Q<T1, T2> 
    where
        T2: PrimIntTrait,
        T2: BrandableT,
        T2: SignIntrospectionTrait,
        T2: MaybeEncodeTrait,
        T2: MaybeDecodeTrait,
        T2: MaybeTypeInfoTrait,
        T2: MaybeStorageLayoutTrait, 
        Precision<T1>: PrecisionTrait {

    pub fn new_from_int<const T3: u8, T4>(v: T4) -> QTraitResult<Self> 
        where 
            T4: PrimIntTrait,
            T4: MaybeEncodeTrait,
            T4: MaybeDecodeTrait,
            T4: MaybeTypeInfoTrait,
            T4: MaybeStorageLayoutTrait,
            Precision<T3>: PrecisionTrait {
        let v: T2 = T2::from(v).ok_or(QTraitError::ConversionFailure)?;
        let v: Q<T3, T2> = q(v);
        let v: Q<T1, T2> = v.cast()?;
        Ok(v)
    }
}

impl<const T1: u8, T2> Q<T1, T2> 
    where
        T2: PrimIntTrait,
        T2: MaybeEncodeTrait,
        T2: MaybeDecodeTrait,
        T2: MaybeTypeInfoTrait,
        T2: MaybeStorageLayoutTrait,
        Precision<T1>: PrecisionTrait, {

    pub fn new(v: T2) -> Self {
        debug_assert!(matches!(T1, 1u8..=38u8));
        Self {
            _v: v,
        }
    }
}

impl<const A: u8, B> Q<A, B> where
    B: PrimIntTrait,
    B: BrandableT,
    B: SignIntrospectionTrait,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    pub fn cast<const C: u8>(&self) -> QTraitResult<Q<C, B>> where
        B: PrimIntTrait,
        B: BrandableT,
        B: SignIntrospectionTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<C>: PrecisionTrait, {
        _mul_div_engine().cast(self, &_mul_div_engine().bit_simulation_algo())
    }
}

impl<const A: u8, B> RemTrait for Q<A, B> where 
    B: PrimIntTrait,
    B: BrandableT,
    B: SignIntrospectionTrait,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    type Output = QTraitResult<Self>;

    fn rem(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        if x.is_signed() && y.is_signed() {
            let v_0: i128 = x._v.to_i128().unwrap();
            let v_1: i128 = y._v.to_i128().unwrap();
            if v_1 == 0i128 {
                return Err(QTraitError::DivisionByZero)
            }
            let v_2: i128 = v_0.checked_rem(v_1).ok_or(QTraitError::RemByZero)?;
            let v_2: B = B::from(v_2).unwrap();
            return Ok(q(v_2))   
        }
        debug_assert!(!x.is_signed());
        debug_assert!(!y.is_signed());
        let v_0: u128 = x._v.to_u128().unwrap();
        let v_1: u128 = y._v.to_u128().unwrap();
        if v_1 == 0u128 {
            return Err(QTraitError::DivisionByZero)
        }
        let v_2: u128 = v_0.checked_rem(v_1).ok_or(QTraitError::RemByZero)?;
        let v_2: B = B::from(v_2).unwrap();
        Ok(q(v_2))
    }
}

impl<const A: u8, B> AddTrait for Q<A, B> where 
    B: PrimIntTrait,
    B: BrandableT,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    type Output = QTraitResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_add(v_1).ok_or(QTraitError::Overflow)?;
        Ok(q(v_2))
    }
}

impl<const A: u8, B> SubTrait for Q<A, B> where 
    B: PrimIntTrait,
    B: BrandableT,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    type Output = QTraitResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_sub(v_1).ok_or(QTraitError::Underflow)?;
        Ok(q(v_2))
    }
}

impl<const A: u8, B> Q<A, B> where
    B: PrimIntTrait,
    B: BrandableT,
    B: SignIntrospectionTrait,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    pub fn mul_with_fallack(self, rhs: Self) -> QTraitResult<Self> {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: MulDivEngineData<'_, A, B> = MulDivEngineData {
            x,
            y,
            algo: &_mul_div_engine().bit_simulation_algo(),
        };
        match _mul_div_engine().mul(data) {
            Ok(result) => Ok(result),
            _ => {
                let data: MulDivEngineData<'_, A, B> = MulDivEngineData {
                    x,
                    y,
                    algo: &_mul_div_engine().native_algo(),
                };
                _mul_div_engine().mul(data)
            } 
        }
    }
}

impl<const A: u8, B> Q<A, B> where
    B: PrimIntTrait,
    B: BrandableT,
    B: SignIntrospectionTrait,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    pub fn mul_with_native(self, rhs: Self) -> QTraitResult<Self> {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: MulDivEngineData<'_, A, B> = MulDivEngineData {
            x,
            y,
            algo: &_mul_div_engine().native_algo(),
        };
        _mul_div_engine().mul(data)
    }
}

impl<const A: u8, B> MulTrait for Q<A, B> where
    B: PrimIntTrait,
    B: BrandableT,
    B: SignIntrospectionTrait,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    type Output = QTraitResult<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: MulDivEngineData<'_, A, B> = MulDivEngineData {
            x,
            y,
            algo: &_mul_div_engine().bit_simulation_algo(),
        };
        _mul_div_engine().mul(data)
    }
}

impl<const A: u8, B> DivTrait for Q<A, B> where
    B: PrimIntTrait,
    B: BrandableT,
    B: SignIntrospectionTrait,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    type Output = QTraitResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let data: mul_div_engine::Data<'_, A, B> = mul_div_engine::Data {
            x,
            y,
            algo: &mul_div_engine::MulDivEngine::default().bit_simulation_algo(),
        };
        mul_div_engine::MulDivEngine::default().div(data)
    }
}

impl<const A: u8, B> Q<A, B> where
    B: PrimIntTrait,
    B: BrandableT,
    B: SignIntrospectionTrait,
    B: MaybeEncodeTrait,
    B: MaybeDecodeTrait,
    B: MaybeTypeInfoTrait,
    B: MaybeStorageLayoutTrait,
    Precision<A>: PrecisionTrait, {

    pub fn max_value(&self) -> B {
        B::max_value()
    }

    pub fn min_value(&self) -> B {
        B::min_value()
    }

    pub fn max_nominal_value(&self) -> B {
        let decimals: u32 = A.into();
        if self.is_signed() {
            let scale: i128 = 10i128.pow(decimals);
            let max_value: i128 = self.max_value().to_i128().unwrap();
            let max_value: i128 = max_value / scale;
            let max_value: B = B::from(max_value).unwrap();
            return max_value
        }
        let scale: u128 = 10u128.pow(decimals);
        let max_value: u128 = self.max_value().to_u128().unwrap();
        let max_value: u128 = max_value / scale;
        let max_value: B = B::from(max_value).unwrap();
        max_value
    }

    pub fn min_nominal_value(&self) -> B {
        let decimals: u32 = A.into();
        if self.is_signed() {
            let scale: i128 = 10i128.pow(decimals);
            let min_value: i128 = self.min_value().to_i128().unwrap();
            let min_value: i128 = min_value / scale;
            let min_value: B = B::from(min_value).unwrap();
            return min_value
        }
        B::zero()
    }
}

impl<const A: u8, B> SignIntrospectionTrait for Q<A, B> 
    where
        B: PrimIntTrait,
        B: SignIntrospectionTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {

    fn is_signed(&self) -> bool {
        self._v.is_signed()
    }
}

impl<const A: u8, B> PartialOrdTrait for Q<A, B> 
    where
        B: PrimIntTrait,
        B: BrandableT,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {

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

impl<const A: u8, B> OrdTrait for Q<A, B> 
    where 
        B: PrimIntTrait,
        B: BrandableT,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {

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
            return q(v_0)
        } 
        q(v_1)
    }

    fn min(self, other: Self) -> Self {
        let x: &Self = &self;
        let y: &Self = &other;
        let v_0: B = x._v;
        let v_1: B = y._v;
        let v_2: B = v_0.min(v_1);
        if v_2 == v_0 {
            return q(v_0)
        }
        q(v_1)    
    }

    fn cmp(&self, other: &Self) -> Ordering {
        let x: &Self = self;
        let y: &Self = other;
        let v_0: B = x._v;
        let v_1: B = y._v;
        v_0.cmp(&v_1)
    }
}

impl<const A: u8, B> PartialEqTrait for Q<A, B> 
    where 
        B: PrimIntTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {

    fn eq(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        v_0 == v_1
    }
}

impl<const A: u8, B> EqTrait for Q<A, B> 
    where 
        B: PrimIntTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, 
{}

#[cfg(not(feature = "ink"))]
pub fn q_fl<const A: u8, const B: u8, C, D>(v: B) -> QTraitResult<Q<B, D>> 
    where
        C: FloatTrait,
        D: PrimIntTrait,
        D: BrandableT,
        D: SignIntrospectionTrait,
        Precision<A>: PrecisionTrait,
        Precision<C>: PrecisionTrait, {
    Q::from_float::<A, C>(v)
}

pub fn q_int<const A: u8, B, const C: u8, D>(v: B) -> QTraitResult<Q::<C, D>> 
    where 
        B: PrimIntTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        D: PrimIntTrait,
        D: BrandableT,
        D: SignIntrospectionTrait,
        D: MaybeEncodeTrait,
        D: MaybeDecodeTrait,
        D: MaybeTypeInfoTrait,
        D: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait,
        Precision<C>: PrecisionTrait, {
    Q::new_from_int::<A, B>(v)
}

pub fn q<const A: u8, B>(v: B) -> Q<A, B> 
    where
        B: PrimIntTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {
    Q::new(v)
}

fn _mul_div_engine() -> MulDivEngine {
    MulDivEngine::default()
}

#[cfg(test)]
mod test {
    boiler::extend!();

    #[test]
    fn test_add_for_unsigned_success() {
        let x: Q<2u8, u128> = q(100u128);
        let y: Q<2u8, u128> = q(100u128);
        let result: Q<2u8, u128> = (x + y).expect("");
        let result_ok: Q<2u8, u128> = q(200u128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_add_for_unsigned_overflow() {
        let max: u128 = u128::MAX;
        let x: Q2<u128> = q(max);
        let y: Q2<u128> = q(100u128);
        (x + y)
            .map_err(|e| assert_eq!(e, QTraitError::Overflow))
            .unwrap_err();
    }

    #[test]
    fn test_add_for_signed_success() {
        let x: Q<2u8, i128> = q(100i128);
        let y: Q<2u8, i128> = q(100i128);
        let result: Q<2u8, i128> = (x + y).expect("");
        let result_ok: Q<2u8, i128> = q(200i128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_add_for_signed_overflow() {
        let max: i128 = i128::MAX;
        let x: Q<2u8, i128> = q(max);
        let y: Q<2u8, i128> = q(100i128);
        (x + y)
            .map_err(|e| assert_eq!(e, QTraitError::Overflow))
            .unwrap_err();
    }

    #[test]
    fn test_sub_for_unsigned_success() {
        let x: Q<2u8, u128> = q(100u128);
        let y: Q<2u8, u128> = q(100u128);
        let result: Q<2u8, u128> = (x - y).unwrap();
        let result_ok: Q<2u8, u128> = q(0u128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_sub_for_unsigned_underflow() {
        let x: Q<2u8, u128> = q(100u128);
        let y: Q<2u8, u128> = q(200u128);
        (x - y)
            .map_err(|e| assert_eq!(e, QTraitError::Underflow))
            .unwrap_err();
    }

    #[test]
    fn test_sub_for_signed_success() {
        let x: Q<2u8, i128> = q(100i128);
        let y: Q<2u8, i128> = q(100i128);
        let result = (x - y).unwrap();
        let result_ok: Q<2u8, i128> = q(0i128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_sub_for_signed_underflow() {
        let min: i128 = i128::MIN;
        let x: Q<2u8, i128> = q(min);
        let y: Q<2u8, i128> = q(100i128);
        (x - y)
            .map_err(|e| assert_eq!(e, QTraitError::Underflow))
            .unwrap_err();
    }

    #[test]
    fn test_mul_for_unsigned_success() {
        let x: Q<2u8, u128> = q(50u128);
        let y: Q<2u8, u128> = q(25u128);
        let result: Q<2u8, u128> = (x * y).unwrap();
        let result_ok: Q<2u8, u128> = q(12u128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_mul_for_unsigned_overflow() {
        let max: u128 = u128::MAX;
        let x: Q<2u8, u128> = q(max);
        let y: Q<2u8, u128> = q(1000u128);
        (x * y)
            .map_err(|e| assert_eq!(e, QTraitError::Overflow))
            .unwrap_err();
    }

    #[test]
    fn test_mul_for_signed_success() {
        let x: Q<2u8, i128> = q(50i128);
        let y: Q<2u8, i128> = q(25i128);
        let result: Q<2u8, i128> = (x * y).unwrap();
        let result_ok: Q<2u8, i128> = q(12i128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_mul_for_signed_overflow() {
        let max: i128 = i128::MAX;
        let x: Q<2u8, i128> = q(max);
        let y: Q<2u8, i128> = q(1000i128);
        (x * y)
            .map_err(|e| assert_eq!(e, QTraitError::Overflow))
            .unwrap_err();
    }

    #[test]
    fn test_div_for_unsigned_success() {
        let x: Q<2u8, u128> = q(50u128);
        let y: Q<2u8, u128> = q(25u128);
        let result: Q<2u8, u128> = (x / y).unwrap();
        let result_ok: Q<2u8, u128> = q(200u128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_div_for_unsigned_division_by_zero() {
        let x: Q<2u8, u128> = q(100u128);
        let y: Q<2u8, u128> = q(0u128);
        (x / y)
            .map_err(|e| assert_eq!(e, QTraitError::DivisionByZero))
            .unwrap_err();
    }

    #[test]
    fn test_div_for_signed_success() {
        let x: Q<2u8, i128> = q(50i128);
        let y: Q<2u8, i128> = q(25i128);
        let result: Q<2u8, i128> = (x / y).unwrap();
        let result_ok: Q<2u8, i128> = q(200i128);
        assert_eq!(result, result_ok);
    }

    #[test]
    fn test_div_for_signed_division_by_zero() {
        let max: i128 = i128::MAX;
        let x: Q<2u8, i128> = q(max);
        let y: Q<2u8, i128> = q(0i128);
        (x / y)
            .map_err(|e| assert_eq!(e, QTraitError::DivisionByZero))
            .unwrap_err();
    }
}