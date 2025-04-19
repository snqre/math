use crate::core::tr_brandable::*;
use crate::core::tr_sign_introspection::*;
use super::*;
use std::ops::Mul;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Mul for Q<A, B> {
    type Output = QR<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if A > Q_MAX_PRECISION {
            return Err(QE::PrecisionTooLarge)
        }
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        if A < Q_MIN_PRECISION {
            let v_2: B = v_0.checked_mul(v_1).ok_or(QE::Overflow)?;
            let v_2: Q<A, B> = q(v_2);
            return Ok(v_2)
        }
        let decimals: u32 = A.into();
        if x.is_signed() && y.is_signed() {
            let v_0: i128 = v_0.to_i128().unwrap();
            let v_1: i128 = v_1.to_i128().unwrap();
            let scale: i128 = 10i128.pow(decimals);
            let v_2: i128 = v_0
                .checked_mul(v_1)
                .ok_or(QE::Overflow)?
                .checked_div(scale)
                .ok_or(QE::DivisionByZero)?;
            if v_2 > x.upper_bound().to_i128().unwrap() {
                return Err(QE::Overflow)
            }
            if v_2 < x.lower_bound().to_i128().unwrap() {
                return Err(QE::Underflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
            return Ok(v_2)
        }
        debug_assert!(!x.is_signed());
        debug_assert!(!y.is_signed());
        let v_0: u128 = v_0.to_u128().unwrap();
        let v_1: u128 = v_1.to_u128().unwrap();
        let scale: u128 = 10u128.pow(decimals);
        let v_2: u128 = v_0
            .checked_mul(v_1)
            .ok_or(QE::Overflow)?
            .checked_div(scale)
            .ok_or(QE::DivisionByZero)?;
        if v_2 > x.upper_bound().to_u128().unwrap() {
            return Err(QE::Overflow)
        }
        let v_2: B = B::from(v_2).unwrap();
        let v_2: Q<A, B> = q(v_2);
        Ok(v_2)
    }
}