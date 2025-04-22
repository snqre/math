use crate::core::cs_k::*;
use crate::core::tr_brandable::*;
use crate::core::tr_sign_introspection::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> K<A, B> {
    pub fn cast<const C: u8>(&self) -> KResult<K<C, B>> {
        use KError::*;
        if A == C {
            return Ok(k(self._v))
        }
        if A > MAX_PRECISION || C > MAX_PRECISION {
            return Err(PrecisionTooLarge)
        }
        let old_decimals: u32 = A.into();
        let new_decimals: u32 = C.into();
        if self.is_signed() {
            let old_scale: i128 = 10i128.pow(old_decimals);
            let new_scale: i128 = 10i128.pow(new_decimals);
            let result: i128 = self._v
                .to_i128()
                .unwrap()
                .checked_mul(new_scale)
                .ok_or(Overflow)?
                .checked_div(old_scale)
                .ok_or(DivisionByZero)?;
            if result > B::max_value().to_i128().unwrap() {
                return Err(Overflow)
            }
            if result < B::min_value().to_i128().unwrap() {
                return Err(Underflow)
            }
            return k_int::<C, i128, C, B>(result)
        }
        debug_assert!(self.is_signed());
        let old_scale: u128 = 10u128.pow(old_decimals);
        let new_scale: u128 = 10u128.pow(new_decimals);
        let result: u128 = self._v
            .to_u128()
            .unwrap()
            .checked_mul(new_scale)
            .ok_or(Overflow)?
            .checked_div(old_scale)
            .ok_or(DivisionByZero)?;
        if result > B::max_value().to_u128().unwrap() {
            return Err(Overflow)
        }
        k_int::<C, u128, C, B>(result)
    }
}