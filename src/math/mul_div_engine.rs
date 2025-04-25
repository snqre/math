use crate::math::q::*;
use crate::math::q_trait::*;
use crate::math::branded_trait::BrandedTrait;
use crate::math::ink::maybe_decode_trait::MaybeDecodeTrait;
use crate::math::ink::maybe_encode_trait::MaybeEncodeTrait;
use crate::math::ink::maybe_storage_layout_trait::MaybeStorageLayoutTrait;
use crate::math::ink::maybe_type_info_trait::MaybeTypeInfoTrait;
use crate::math::sign_introspection_trait::SignIntrospectionTrait;
use crate::math::precision::Precision;
use crate::math::precision_trait::PrecisionTrait;
use num_traits::int::PrimInt as PrimIntTrait;

pub type MulDivEngineAlgo = (MulDivEngineU128Algo, MulDivEngineI128Algo);

pub type MulDivEngineU128Algo = fn(u128, u128, u128) -> QTraitResult<u128>;

pub type MulDivEngineI128Algo = fn(i128, i128, i128) -> QTraitResult<i128>;

pub struct MulDivEngineData<'a, const A: u8, B> 
    where
        B: PrimIntTrait,
        B: BrandedTrait,
        B: SignIntrospectionTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {
    pub x: &'a Q<A, B>,
    pub y: &'a Q<A, B>,
    pub algo: &'a MulDivEngineAlgo,
}

#[derive(Debug)]
pub struct MulDivEngine;

impl MulDivEngine {

    pub fn default() -> Self {
        Self {}
    }

    pub fn bit_simulation_algo(&self) -> MulDivEngineAlgo {
        (_mul_div_u128_with_256_bit_simulation, _mul_div_i128_with_256_bit_simulation)
    }

    pub fn native_algo(&self) -> MulDivEngineAlgo {
        (_mul_div_u128_with_native, _mul_div_i128_with_native)
    }

    pub fn cast<const A: u8, const B: u8, C>(&self, current: &Q<A, C>, algo: &MulDivEngineAlgo) -> QTraitResult<Q<B, C>> 
        where
            C: PrimIntTrait,
            C: BrandedTrait,
            C: SignIntrospectionTrait,
            C: MaybeEncodeTrait,
            C: MaybeDecodeTrait,
            C: MaybeTypeInfoTrait,
            C: MaybeStorageLayoutTrait,
            Precision<A>: PrecisionTrait,
            Precision<B>: PrecisionTrait, {
        if A == B {
            let current: C = current._v;
            let current: Q<B, C> = q(current);
            return Ok(current)
        }
        let old_decimals: u32 = A.into();
        let new_decimals: u32 = B.into();
        if current.is_signed() {
            let old_scale: i128 = 10i128.pow(old_decimals);
            let new_scale: i128 = 10i128.pow(new_decimals);
            let result: i128 = current._v.to_i128().unwrap();
            let result: i128 = (algo.1)(result, new_scale, old_scale)?;
            if result > C::max_value().to_i128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            if result < C::min_value().to_i128().unwrap() {
                return Err(QTraitError::Underflow)
            }
            return q_int::<A, i128, B, C>(result)
        }
        let old_scale: u128 = 10u128.pow(old_decimals);
        let new_scale: u128 = 10u128.pow(new_decimals);
        let result: u128 = current._v.to_u128().unwrap();
        let result: u128 = (algo.0)(result, new_scale, old_scale)?;
        if result > C::max_value().to_u128().unwrap() {
            return Err(QTraitError::Overflow)
        }
        q_int::<A, u128, B, C>(result)
    }

    pub fn mul<const A: u8, B>(&self, data: MulDivEngineData<'_, A, B>) -> QTraitResult<Q<A, B>> where 
        B: PrimIntTrait,
        B: BrandedTrait,
        B: SignIntrospectionTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {
        let decimals: u32 = A.into();
        let scale: u128 = 10u128.pow(decimals);
        if data.x.is_signed() && data.y.is_signed() {
            let scale: i128 = scale.try_into().unwrap();
            let v_0: i128 = data.x._v.to_i128().unwrap();
            let v_1: i128 = data.y._v.to_i128().unwrap();
            let result: i128 = (data.algo.1)(v_0, v_1, scale)?;
            if result > data.x.max_value().to_i128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            if result < data.x.min_value().to_i128().unwrap() {
                return Err(QTraitError::Underflow)
            }
            let result: B = B::from(result).unwrap();
            return Ok(q(result))
        }
        debug_assert!(!data.x.is_signed());
        debug_assert!(!data.y.is_signed());
        let v_0: u128 = data.x._v.to_u128().unwrap();
        let v_1: u128 = data.y._v.to_u128().unwrap();
        let result: u128 = (data.algo.0)(v_0, v_1, scale)?;
        if result > data.x.max_value().to_u128().unwrap() {
            return Err(QTraitError::Overflow)
        }
        let result: B = B::from(result).unwrap();
        Ok(q(result))
    }

    pub fn div<const A: u8, B>(&self, data: MulDivEngineData<'_, A, B>) -> QTraitResult<Q<A, B>> where
        B: PrimIntTrait,
        B: BrandedTrait,
        B: SignIntrospectionTrait,
        B: MaybeEncodeTrait,
        B: MaybeDecodeTrait,
        B: MaybeTypeInfoTrait,
        B: MaybeStorageLayoutTrait,
        Precision<A>: PrecisionTrait, {
        let decimals: u32 = A.into();
        let scale: u128 = 10u128.pow(decimals);
        if data.x.is_signed() && data.y.is_signed() {
            let v_0: i128 = data.x._v.to_i128().unwrap();
            let v_1: i128 = data.y._v.to_i128().unwrap();
            let result: i128 = if scale.is_power_of_two() {
                let scale_shift: u32 = scale.trailing_zeros();
                (v_0 << scale_shift).checked_div(v_1).ok_or(QTraitError::DivisionByZero)?
            } else {
                let scale: i128 = scale.try_into().unwrap();
                (data.algo.1)(v_0, scale, v_1)?
            };
            if result > data.x.max_value().to_i128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            if result < data.x.min_value().to_i128().unwrap() {
                return Err(QTraitError::Underflow)
            }
            let result: B = B::from(result).unwrap();
            let result: Q<A, B> = q(result);
            return Ok(result)
        }
        debug_assert!(!data.x.is_signed());
        debug_assert!(!data.y.is_signed());
        let v_0: u128 = data.x._v.to_u128().unwrap();
        let v_1: u128 = data.y._v.to_u128().unwrap();
        let result: u128 = if scale.is_power_of_two() {
            let scale_shift: u32 = scale.trailing_zeros();
            (v_0 << scale_shift).checked_div(v_1).ok_or(QTraitError::DivisionByZero)?
        } else {
            (data.algo.0)(v_0, scale, v_1)?
        };
        if result > data.x.max_value().to_u128().unwrap() {
            return Err(QTraitError::Overflow)
        }
        let result: B = B::from(result).unwrap();
        let result: Q<A, B> = q(result);
        Ok(result)
    }
}

fn _mul_div_u128_with_native(x: u128, y: u128, z: u128) -> QTraitResult<u128> {
    if z == 0u128 {
        return Err(QTraitError::DivisionByZero)
    }
    let result: u128 = x.checked_mul(y).ok_or(QTraitError::Overflow)?;
    let result: u128 = result / z;
    Ok(result)
}

fn _mul_div_i128_with_native(x: i128, y: i128, z: i128) -> QTraitResult<i128> {
    if z == 0 {
        return Err(QTraitError::DivisionByZero)
    }
    let sign: i128 = ((x ^ y ^ z) >> 127) & 1;
    let x_u: u128 = x.unsigned_abs();
    let y_u: u128 = y.unsigned_abs();
    let z_u: u128 = z.unsigned_abs();
    let result = x_u.checked_mul(y_u).ok_or(QTraitError::Overflow)?;
    let result = result / z_u;
    let result = if sign == 1 {
        result.wrapping_neg() as i128
    } else {
        result as i128
    };
    Ok(result)
}

fn _mul_div_u128_with_256_bit_simulation(x: u128, y: u128, z: u128) -> QTraitResult<u128> {
    if z == 0 {
        return Err(QTraitError::DivisionByZero)
    }
    let x_hi: u128 = x >> 64;
    let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
    let y_hi: u128 = y >> 64;
    let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
    let lo_lo: u128 = x_lo * y_lo;
    let lo_hi: u128 = x_lo * y_hi;
    let hi_lo: u128 = x_hi * y_lo;
    let hi_hi: u128 = x_hi * y_hi;
    let m: u128 = lo_hi + hi_lo;
    let c_0: u128 = ((m < lo_hi) as u128) << 64;
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
        return Err(QTraitError::Overflow)
    }
    let rem: u128 = ((p_1 % z) << 64) | (p_0 >> 64);
    let rem: u128 = ((rem % z) << 64) | (p_0 & 0xFFFFFFFFFFFFFFFF);
    let result: u128 = rem / z;
    Ok(result)
}

fn _mul_div_i128_with_256_bit_simulation(x: i128, y: i128, z: i128) -> QTraitResult<i128> {
    if z == 0 {
        return Err(QTraitError::DivisionByZero)
    }
    let sign: i128 = ((x ^ y ^ z) >> 127) & 1;
    let x_u: u128 = x.unsigned_abs();
    let y_u: u128 = y.unsigned_abs();
    let z_u: u128 = z.unsigned_abs();
    let x_hi: u128 = x_u >> 64;
    let x_lo: u128 = x_u & 0xFFFFFFFFFFFFFFFF;
    let y_hi: u128 = y_u >> 64;
    let y_lo: u128 = y_u & 0xFFFFFFFFFFFFFFFF;
    let lo_lo: u128 = x_lo * y_lo;
    let lo_hi: u128 = x_lo * y_hi;
    let hi_lo: u128 = x_hi * y_lo;
    let hi_hi: u128 = x_hi * y_hi;
    let m: u128 = lo_hi + hi_lo;
    let c_0: u128 = ((m < lo_hi) as u128) << 64;
    let m_lo: u128 = m << 64;
    let m_hi: u128 = m >> 64;
    let p_0: u128 = lo_lo.wrapping_add(m_lo);
    let c_1: u128 = (p_0 < lo_lo) as u128;
    let p_1: u128 = hi_hi + m_hi + c_0 + c_1;
    if p_1 == 0 {
        let result: u128 = p_0 / z_u;
        let result: i128 = if sign == 1 {
            result.wrapping_neg() as i128
        } else {
            result as i128
        };
        return Ok(result)
    }
    if p_1 >= z_u {
        return Err(QTraitError::Overflow)
    }
    let rem: u128 = ((p_1 % z_u) << 64) | (p_0 >> 64);
    let rem: u128 = ((rem % z_u) << 64) | (p_0 & 0xFFFFFFFFFFFFFFFF);
    let result: u128 = rem / z_u;
    let result: i128 = if sign == 1 {
        result.wrapping_neg() as i128
    } else {
        result as i128
    };
    Ok(result)
}