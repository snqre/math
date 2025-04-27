use crate::math::Q;
use crate::math::Q::new as q;
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
use num_traits::int::PrimInt as PrimInt;

const SIGNED_SCALES: [i128; 38usize] = [
    10i128.pow(1u32),
    10i128.pow(2u32),
    10i128.pow(3u32),
    10i128.pow(4u32),
    10i128.pow(5u32),
    10i128.pow(6u32),
    10i128.pow(7u32),
    10i128.pow(8u32),
    10i128.pow(9u32),
    10i128.pow(10u32),
    10i128.pow(11u32),
    10i128.pow(12u32),
    10i128.pow(13u32),
    10i128.pow(14u32),
    10i128.pow(15u32),
    10i128.pow(16u32),
    10i128.pow(17u32),
    10i128.pow(18u32),
    10i128.pow(19u32),
    10i128.pow(20u32),
    10i128.pow(21u32),
    10i128.pow(22u32),
    10i128.pow(23u32),
    10i128.pow(24u32),
    10i128.pow(25u32),
    10i128.pow(26u32),
    10i128.pow(27u32),
    10i128.pow(28u32),
    10i128.pow(29u32),
    10i128.pow(30u32),
    10i128.pow(31u32),
    10i128.pow(32u32),
    10i128.pow(33u32),
    10i128.pow(34u32),
    10i128.pow(35u32),
    10i128.pow(36u32),
    10i128.pow(37u32),
    10i128.pow(38u32)
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
pub struct SimpleMulDivEngine;

impl Q::IsMulDivEngine for SimpleMulDivEngine {

    fn algorithm(&self) -> Q::Algorithm {
        (_mul_div_u128, _mul_div_i128)
    }

    fn cast<const A: u8, const B: u8, C: IsInt + HasSignIntrospection, D: Q::IsMulDivEngine>(&self, current: &Q::Q<A, C, D>, algorithm: &Q::Algorithm) -> Q::Result<Q::Q<B, C, D>> 
        where
            Precision<A>: IsCompatiblePrecision,
            Precision<B>: IsCompatiblePrecision {
        if A == B {
            let engine: D = current._engine.clone();
            let current: C = current._v;
            let current: Q::Q<B, C, D> = Q::new_with_custom_engine(current, engine);
            return Ok(current)
        }
        if current.is_signed() {
            let old_scale: i128 = SIGNED_SCALES[A as usize];
            let new_scale: i128 = SIGNED_SCALES[B as usize];
            let result: i128 = current._v.to_i128().unwrap();
            let result: i128 = (algorithm.1)(result, new_scale, old_scale)?;
            if result > C::max_value().to_i128().unwrap() {
                return Err(Q::Error::Overflow)
            }
            if result < C::min_value().to_i128().unwrap() {
                return Err(Q::Error::Underflow)
            }
            let result: C = C::from(result).unwrap();
            let engine: D = current._engine.clone();
            let result: Q::Q<B, C, D> = Q::new_with_custom_engine(result, engine);
            return Ok(result)
        }
        let old_scale: u128 = UNSIGNED_SCALES[A as usize];
        let new_scale: u128 = UNSIGNED_SCALES[B as usize];
        let result: u128 = current._v.to_u128().unwrap();
        let result: u128 = (algorithm.0)(result, new_scale, old_scale)?;
        if result > C::max_value().to_u128().unwrap() {
            return Err(Q::Error::Overflow)
        }
        let result: C = C::from(result).unwrap();
        let engine: D = current._engine.clone();
        let result: Q::Q<B, C, D> = Q::new_with_custom_engine(result, engine);
        Ok(result)
    }

    fn mul<const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: Q::IsMulDivEngine>(&self, data: &Q::Data<A, B, C>) -> Q::Result<Q::Q<A, B, C>> where Precision<A>: IsCompatiblePrecision {
        if data.x.is_signed() && data.y.is_signed() {
            let scale: i128 = SIGNED_SCALES[A as usize];
            let v_0: i128 = data.x.to_i128().unwrap();
            let v_1: i128 = data.y.to_i128().unwrap();
            let result: i128 = (data.algorithm.1)(v_0, v_1, scale)?;
            if result > data.x.max_underlying_value().to_i128().unwrap() {
                return Err(Q::Error::Overflow)
            }
            if result < data.x.min_underlying_value().to_i128().unwrap() {
                return Err(Q::Error::Underflow)
            }
            let engine: C = data.x._engine.clone();
            let result: B = B::from(result).unwrap();
            let result: Q::Q<A, B, C> = Q::new_with_custom_engine(result, engine);
            return Ok(result)
        }
        debug_assert!(!data.x.is_signed());
        debug_assert!(!data.y.is_signed());
        let scale: u128 = UNSIGNED_SCALES[A as usize];
        let v_0: u128 = data.x.to_u128().unwrap();
        let v_1: u128 = data.y.to_u128().unwrap();
        let result: u128 = (data.algorithm.0)(v_0, v_1, scale)?;
        if result > data.x.max_underlying_value().to_u128().unwrap() {
            return Err(Q::Error::Overflow)
        }
        let engine: C = data.x._engine.clone();
        let result: B = B::from(result).unwrap();
        let result: Q::Q<A, B, C> = Q::new_with_custom_engine(result, engine);
        Ok(result)
    }

    fn div<const A: u8, B: IsInt + HasBrand + HasSignIntrospection, C: Q::IsMulDivEngine>(&self, data: &Q::Data<'_, A, B, C>) -> Q::Result<Q::Q<A, B, C>> where Precision<A>: IsCompatiblePrecision {
        let scale: u128 = UNSIGNED_SCALES[A as usize];
        if data.x.is_signed() && data.y.is_signed() {
            let v_0: i128 = data.x.to_i128().unwrap();
            let v_1: i128 = data.y.to_i128().unwrap();
            let result: i128 = if scale.is_power_of_two() {
                let scale_shift: u32 = scale.trailing_zeros();
                (v_0 << scale_shift).checked_div(v_1).ok_or(Q::Error::DivByZero)?
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
            let engine: C = data.x._engine.clone();
            let result: B = B::from(result).unwrap();
            let result: Q::Q<A, B, C> = Q::new_with_custom_engine(result, engine);
            return Ok(result)
        }
        debug_assert!(!data.x.is_signed());
        debug_assert!(!data.y.is_signed());
        let v_0: u128 = data.x.to_u128().unwrap();
        let v_1: u128 = data.y.to_u128().unwrap();
        let result: u128 = if scale.is_power_of_two() {
            let scale_shift: u32 = scale.trailing_zeros();
            (v_0 << scale_shift).checked_div(v_1).ok_or(Q::Error::DivByZero)?
        } else {
            (data.algorithm.0)(v_0, scale, v_1)?
        };
        if result > data.x.max_underlying_value().to_u128().unwrap() {
            return Err(Q::Error::Overflow)
        }
        let engine: C = data.x._engine.clone();
        let result: B = B::from(result).unwrap();
        let result: Q::Q<A, B, C> = Q::new_with_custom_engine(result, engine);
        Ok(result)
    }
}

pub fn default() -> SimpleMulDivEngine {
    SimpleMulDivEngine
}

fn _mul_div_u128(x: u128, y: u128, z: u128) -> Q::Result<u128> {
    use Q::Error::*;
    if z == 0u128 {
        return Err(DivByZero)
    }
    let result: u128 = x.checked_mul(y).ok_or(Overflow)?;
    let result: u128 = result / z;
    Ok(result)
}

fn _mul_div_i128(x: i128, y: i128, z: i128) -> Q::Result<i128> {
    use Q::Error::*;
    if z == 0 {
        return Err(DivByZero)
    } 
    let sign: i128 = ((x ^ y ^ z) >> 127) & 1;
    let x_u: u128 = x.unsigned_abs();
    let y_u: u128 = y.unsigned_abs();
    let z_u: u128 = z.unsigned_abs();
    let result = x_u.checked_mul(y_u).ok_or(Overflow)?;
    let result = result / z_u;
    let result = if sign == 1 {
        result.wrapping_neg() as i128
    } else {
        result as i128
    };
    Ok(result)
}