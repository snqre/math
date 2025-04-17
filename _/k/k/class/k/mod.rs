use super::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::Ordering;
use std::any::type_name;
use k_branded::Branded;
use thiserror::Error;
use num_traits::int::PrimInt;



pub struct K<const A: u8, B: PrimInt> {
    _v: B,
}

impl<const A: u8, B: PrimInt + Branded> K<A, B> {




    fn _wp<C: PrimInt>(v: C) -> KResult<Self> {

    }

    fn _wi128(v: i128) -> KResult<Self> {
        let max: i128 = B::max_value().to_i128().unwrap();
        let min: i128 = B::min_value().to_i128().unwrap();
        let v_2_le_max: bool = v <= max;
        let v_2_ge_min: bool = v >= min;
        if !v_2_le_max {
            let e: KError = KError::Overflow;
            Err(e)
        } else if !v_2_ge_min {
            let e: KError = KError::Underflow;
            Err(e)
        } else {
            let v: B = B::from(v).unwrap();
            Ok(k(v))
        }
    }

    fn _wu128(v: u128) -> KResult<Self> {
        let max: u128 = B::max_value().to_u128().unwrap();
        let min: u128 = B::min_value().to_u128().unwrap();
        let v_2_le_max: bool = v <= max;
        let v_2_ge_min: bool = v >= min;
        if !v_2_le_max {
            let e: KError = KError::Overflow;
            Err(e)
        } else if !v_2_ge_min {
            let e: KError = KError::Underflow;
            Err(e)
        } else {
            let v: B = B::from(v).unwrap();
            Ok(k(v))
        }
    }

    fn _wu64(v: u64) -> KResult<Self> {
        let max: u64 = B::max_value().to_u64().unwrap();
        let min: u64 = B::min_value().to_u128().unwrap();
        let v_2_le_max: bool = v <= max;
        let v_2_ge_min: bool = v >= min;
        if !v_2_le_max {
            let e: KError = KError::Overflow;
            Err(e)
        } else if !v_2_ge_min {
            let e: KError = KError::Underflow;
            Err(e)
        } else {
            let v: B = B::from(v).unwrap();
            Ok(k(v))
        }
    }
}

pub fn k<const A: u8, B: PrimInt>(v: B) -> K::<A, B> {
    K::new(v)
}

fn _only_safe_precision(decimals: u8) -> KResult<u8> {
    if decimals > K_MAX_DECIMALS {
        return Err(KError::PrecisionTooLarge)
    }
    if decimals < K_MIN_DECIMALS {
        return Err(KError::PrecisionTooSmall)
    }
    Ok(decimals)
}

fn _muldiv<C: PrimInt>(x: C, y: C, z: C) -> KResult<C> {
    let v: C = x
        .checked_mul(&y)
        .ok_or(KError::Overflow)?
        .checked_div(&z)
        .ok_or(KError::DivisionByZero)?;
    Ok(v)
}

mod add;
mod cap;
mod cast;
mod constructor;
mod display;
mod div;
mod eq;
mod int_classifier;
mod mul;
mod ord;
mod partial_eq;
mod partial_ord;
mod representable_cap;
mod sub;
mod to_f32;
mod to_f64;
mod to_u32;
mod to_u64;
mod to_u128;
mod variant;