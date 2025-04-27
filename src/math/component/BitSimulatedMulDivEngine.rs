use crate::math::Q;
use crate::math::Q::new as q;
use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;
use crate::math::util_trait::ink::maybe_has_decode::MaybeHasDecode;
use crate::math::util_trait::ink::maybe_has_encode::MaybeHasEncode;
use crate::math::util_trait::ink::maybe_has_storage_layout::MaybeHasStorageLayout;
use crate::math::util_trait::ink::maybe_has_type_info::MaybeHasTypeInfo;
use crate::math::util::is_compatible_precision::IsCompatiblePrecision;
use crate::math::util::precision::Precision;
use num_traits::int::PrimInt as PrimInt;

const SIGNED_SCALES: [i128, 38usize] = [

];

const UNSIGNED_SCALES: [u128; 38usize] = [
    10u128.pow(1u32),
    10u128.pow(2u32),
    10u128.pow(3u32),
    10u128.pow(4u32),
    10u128.pow(5u32),
    10u128.pow(6u32),
    10u128.pow(7u32),
    10u128.pow(8u32),
    10u128.pow(9u32),
    10u128.pow(10u32),
    10u128.pow(11u32),
    10u128.pow(12u32),
    10u128.pow(13u32),
    10u128.pow(14u32),
    10u128.pow(15u32),
    10u128.pow(16u32),
    10u128.pow(17u32),
    10u128.pow(18u32),
    10u128.pow(19u32),
    10u128.pow(20u32),
    10u128.pow(21u32),
    10u128.pow(22u32),
    10u128.pow(23u32),
    10u128.pow(24u32),
    10u128.pow(25u32),
    10u128.pow(26u32),
    10u128.pow(27u32),
    10u128.pow(28u32),
    10u128.pow(29u32),
    10u128.pow(30u32),
    10u128.pow(31u32),
    10u128.pow(32u32),
    10u128.pow(33u32),
    10u128.pow(34u32),
    10u128.pow(35u32),
    10u128.pow(36u32),
    10u128.pow(37u32),
    10u128.pow(38u32)
];

#[derive(Debug)]
#[derive(Clone)]
pub struct BitSimulatedMulDivEngine;

impl Q::IsMulDivEngine for BitSimulatedMulDivEngine {

    fn algorithm(&self) -> Q::Algorithm {
        (_mul_div_u128_with_256_bit_simulation, _mul_div_i128_with_256_bit_simulation)
    }

    fn cast<const A: u8, const B: u8, C, D>(&self, current: &Q::Q<A, C, D>, algo: &Q::Algorithm) -> Q::Result<Q::Q<B, C, D>> 
        where
            C: PrimInt,
            C: HasBrand,
            C: HasSignIntrospection,
            C: MaybeHasDecode,
            C: MaybeHasEncode,
            C: MaybeHasStorageLayout,
            C: MaybeHasTypeInfo,
            D: Q::IsMulDivEngine,
            Precision<A>: IsCompatiblePrecision,
            Precision<B>: IsCompatiblePrecision, {
        if A == B {
            let engine: D = current._engine.clone();
            let current: C = current._v;
            let current: Q::Q<B, C, D> = Q::new_with_custom_engine(current, engine);
            return Ok(current)
        }
        let old_decimals: u32 = A.into();
        let new_decimals: u32 = B.into();
        if current.is_signed() {
            let old_scale: i128 = 10i128.pow(old_decimals);
            let new_scale: i128 = 10i128.pow(new_decimals);
            let engine: D = current._engine.clone();
            let result: i128 = current._v.to_i128().unwrap();
            let result: i128 = (algo.1)(result, new_scale, old_scale)?;
            if result > C::max_value().to_i128().unwrap() {
                return Err(Q::Error::Overflow)
            }
            if result < C::min_value().to_i128().unwrap() {
                return Err(Q::Error::Underflow)
            }
            let result: C = C::from(result).unwrap();
            let result: Q::Q<B, C, D> = Q::new_with_custom_engine(result, engine);
            return Ok(result)
        }
        let old_scale: u128 = 10u128.pow(old_decimals);
        let new_scale: u128 = 10u128.pow(new_decimals);
        let engine: D = current._engine.clone();
        let result: u128 = current._v.to_u128().unwrap();
        let result: u128 = (algo.0)(result, new_scale, old_scale)?;
        if result > C::max_value().to_u128().unwrap() {
            return Err(Q::Error::Overflow)
        }
        let result: C = C::from(result).unwrap();
        let result: Q::Q<B, C, D> = Q::new_with_custom_engine(result, engine);
        Ok(result)
    }

    fn mul<const T_PRECISION: u8, TInteger, TEngine>(&self, data: &Q::Data<T_PRECISION, TInteger, TEngine>) -> Q::Result<Q::Q<T_PRECISION, TInteger, TEngine>>
        where
            TInteger: PrimInt,
            TInteger: HasBrand,
            TInteger: HasSignIntrospection,
            TInteger: MaybeHasDecode,
            TInteger: MaybeHasEncode,
            TInteger: MaybeHasStorageLayout,
            TInteger: MaybeHasTypeInfo,
            TEngine: Q::IsMulDivEngine,
            Precision<T_PRECISION>: IsCompatiblePrecision {
        let scale: u128 = UNSIGNED_SCALES[T_PRECISION as usize];
        if data.x.is_signed() && data.y.is_signed() {
            let scale: i128 = scale.try_into().unwrap();
            let v_0: i128 = data.x.to_i128().unwrap();
            let v_1: i128 = data.y.to_i128().unwrap();
            let result: i128 = (data.algorithm.1)(v_0, v_1, scale)?;
            if result > data.x.max_underlying_value().to_i128().unwrap() {
                return Err(Q::Error::Overflow)
            }
            if result < data.x.min_underlying_value().to_i128().unwrap() {
                return Err(Q::Error::Underflow)
            }
            let engine: TEngine = data.x._engine.clone();
            let result: TInteger = TInteger::from(result).unwrap();
            let result: Q::Q<T_PRECISION, TInteger, TEngine> = Q::new_with_custom_engine(result, engine);
            return Ok(result)
        }
        debug_assert!(!data.x.is_signed());
        debug_assert!(!data.y.is_signed());
        let v_0: u128 = data.x.to_u128().unwrap();
        let v_1: u128 = data.y.to_u128().unwrap();
        let result: u128 = (data.algorithm.0)(v_0, v_1, scale)?;
        if result > data.x.max_underlying_value().to_u128().unwrap() {
            return Err(Q::Error::Overflow)
        }
        let engine: TEngine = data.x._engine.clone();
        let result: TInteger = TInteger::from(result).unwrap();
        let result: Q::Q<T_PRECISION, TInteger, TEngine> = Q::new_with_custom_engine(result, engine);
        Ok(result)
    }

    fn div<const T_PRECISION: u8, TInteger, TEngine>(&self, data: &Q::Data<'_, T_PRECISION, TInteger, TEngine>) -> Q::Result<Q::Q<T_PRECISION, TInteger, TEngine>>
        where
            TInteger: PrimInt,
            TInteger: HasBrand,
            TInteger: HasSignIntrospection,
            TInteger: MaybeHasDecode,
            TInteger: MaybeHasEncode,
            TInteger: MaybeHasStorageLayout,
            TInteger: MaybeHasTypeInfo,
            TEngine: Q::IsMulDivEngine,
            Precision<T_PRECISION>: IsCompatiblePrecision {
        let scale: u128 = UNSIGNED_SCALES[T_PRECISION as usize];
        if data.x.is_signed() && data.y.is_signed() {
            let v_0: i128 = data.x.to_i128().unwrap();
            let v_1: i128 = data.y.to_i128().unwrap();
            let result: i128 = if scale.is_power_of_two() {
                let scale_shift: u32 = scale.trailing_zeros();
                (v_0 << scale_shift).checked_div(v_1).ok_or(Q::Error::DivisionByZero)?
            } else {
                let scale: i128 = scale.try_into().unwrap();
                (data.algorithm.1)(v_0, scale, v_1)?
            };
            if result > data.x.max_underlying_value().to_i128().unwrap() {
                return Err(Q::Error::Overflow)
            }
            if result < data.x.min_underlying_value().to_i128().unwrap() {
                return Err(Q::Error::Underflow)
            }
            let engine: TEngine = data.x._engine.clone();
            let result: TInteger = TInteger::from(result).unwrap();
            let result: Q::Q<T_PRECISION, TInteger, TEngine> = Q::new_with_custom_engine(result, engine);
            return Ok(result)
        }
        debug_assert!(!data.x.is_signed());
        debug_assert!(!data.y.is_signed());
        let v_0: u128 = data.x.to_u128().unwrap();
        let v_1: u128 = data.y.to_u128().unwrap();
        let result: u128 = if scale.is_power_of_two() {
            let scale_shift: u32 = scale.trailing_zeros();
            (v_0 << scale_shift).checked_div(v_1).ok_or(Q::Error::DivisionByZero)?
        } else {
            (data.algorithm.0)(v_0, scale, v_1)?
        };
        if result > data.x.max_underlying_value().to_u128().unwrap() {
            return Err(Q::Error::Overflow)
        }
        let engine: TEngine = data.x._engine.clone();
        let result: TInteger = TInteger::from(result).unwrap();
        let result: Q::Q<T_PRECISION, TInteger, TEngine> = Q::new_with_custom_engine(result, engine);
        Ok(result)
    }
}

pub fn default() -> BitSimulatedMulDivEngine {
    BitSimulatedMulDivEngine
}

fn _mul_div_u128_with_256_bit_simulation(x: u128, y: u128, z: u128) -> Q::Result<u128> {
    use Q::Error::*;
    if z == 0 {
        return Err(DivisionByZero)
    }
    let x_hi: u128 = x >> 64u128;
    let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
    let y_hi: u128 = y >> 64u128;
    let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
    let lo_lo: u128 = x_lo * y_lo;
    let lo_hi: u128 = x_lo * y_hi;
    let hi_lo: u128 = x_hi * y_lo;
    let hi_hi: u128 = x_hi * y_hi;
    let m: u128 = lo_hi + hi_lo;
    let c_0: u128 = ((m < lo_hi) as u128) << 64u128;
    let m_lo: u128 = m << 64;
    let m_hi: u128 = m >> 64;
    let p_0: u128 = lo_lo.wrapping_add(m_lo);
    let c_1: u128 = (p_0 < lo_lo) as u128;
    let p_1: u128 = hi_hi + m_hi + c_0 + c_1;
    if p_1 == 0 {
        let result: u128 = p_0 / z;
        return Ok(result)
    }
    if p_1 >= z {
        return Err(Overflow)
    }
    let rem: u128 = ((p_1 % z) << 64u128) | (p_0 >> 64u128);
    let rem: u128 = ((rem % z) << 64u128) | (p_0 & 0xFFFFFFFFFFFFFFFF);
    let result: u128 = rem / z;
    Ok(result)
}

fn _mul_div_i128_with_256_bit_simulation(x: i128, y: i128, z: i128) -> Q::Result<i128> {
    use Q::Error::*;
    if z == 0 {
        return Err(DivisionByZero)
    }
    let sign: i128 = ((x ^ y ^ z) >> 127i128) & 1i128;
    let x: u128 = x.unsigned_abs();
    let y: u128 = y.unsigned_abs();
    let z: u128 = z.unsigned_abs();
    let x_hi: u128 = x >> 64u128;
    let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
    let y_hi: u128 = y >> 64u128;
    let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
    let lo_lo: u128 = x_lo * y_lo;
    let lo_hi: u128 = x_lo * y_hi;
    let hi_lo: u128 = x_hi * y_lo;
    let hi_hi: u128 = x_hi * y_hi;
    let m: u128 = lo_hi + hi_lo;
    let c_0: u128 = ((m < lo_hi) as u128) << 64u128;
    let m_lo: u128 = m << 64u128;
    let m_hi: u128 = m >> 64u128;
    let p_0: u128 = lo_lo.wrapping_add(m_lo);
    let c_1: u128 = (p_0 < lo_lo) as u128;
    let p_1: u128 = hi_hi + m_hi + c_0 + c_1;
    if p_1 == 0u128 {
        let result: u128 = p_0 / z;
        let result: i128 = if sign == 1i128 {
            result.wrapping_neg() as i128
        } else {
            result as i128
        };
        return Ok(result)
    }
    if p_1 >= z {
        return Err(Overflow)
    }
    let rem: u128 = ((p_1 % z) << 64u128) | (p_0 >> 64u128);
    let rem: u128 = ((rem % z) << 64u128) | (p_0 & 0xFFFFFFFFFFFFFFFF);
    let result: u128 = rem / z;
    let result: i128 = if sign == 1i128 {
        result.wrapping_neg() as i128
    } else {
        result as i128
    };
    Ok(result)
}