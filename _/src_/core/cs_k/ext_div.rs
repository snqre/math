use crate::core::cs_k::*;
use crate::core::tr_brandable::*;
use crate::core::tr_sign_introspection::*;
use std::ops::Div;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Div for K<A, B> {
    type Output = KResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        use KError::*;
        if A > MAX_PRECISION {
            return Err(PrecisionTooLarge)
        }
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        if A < MIN_PRECISION {
            let v_2: B = v_0.checked_div(v_1).ok_or(DivisionByZero)?;
            let v_2: K<A, B> = k(v_2);
            return Ok(v_2)
        }
        let decimals: u32 = A.into();
        let scale: u128 = 10u128.pow(decimals);
        if x.is_signed() && y.is_signed() {
            let v_0: i128 = v_0.to_i128().unwrap();
            let v_1: i128 = v_1.to_i128().unwrap();
            let v_2: i128;
            if scale.is_power_of_two() {
                let scale_shift: u32 = scale.trailing_zeros();
                v_2 = v_0
                    .checked_shl(scale_shift)
                    .unwrap()
                    .checked_div(v_1)
                    .ok_or(DivisionByZero)?;
            } else {
                let scale: i128 = scale.try_into().unwrap();
                v_2 = v_0
                    .checked_mul(scale)
                    .ok_or(Overflow)?
                    .checked_div(v_1)
                    .ok_or(DivisionByZero)?;
            }
            if v_2 > x.upper_bound().to_i128().unwrap() {
                return Err(Overflow)
            }
            if v_2 < x.lower_bound().to_i128().unwrap() {
                return Err(Underflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: K<A, B> = k(v_2);
            return Ok(v_2)
        }
        debug_assert!(!x.is_signed());
        debug_assert!(!y.is_signed());
        let v_0: u128 = v_0.to_u128().unwrap();
        let v_1: u128 = v_1.to_u128().unwrap();
        let v_2: u128;
        if scale.is_power_of_two() {
            let scale_shift: u32 = scale.trailing_zeros();
            v_2 = v_0
                .checked_shl(scale_shift)
                .unwrap()
                .checked_div(v_1)
                .ok_or(DivisionByZero)?;
        } else {
            v_2 = v_1
                .checked_mul(scale)
                .ok_or(Overflow)?
                .checked_div(v_1)
                .ok_or(DivisionByZero)?;
        }
        if v_2 > x.upper_bound().to_u128().unwrap() {
            return Err(Overflow)
        }
        let v_2: B = B::from(v_2).unwrap();
        let v_2: K<A, B> = k(v_2);
        Ok(v_2)
    }
}