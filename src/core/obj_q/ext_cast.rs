boiler::extend!();

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> q<A, B> {
    pub fn cast<const C: u8>(&self) -> QR<q<C, B>> {
        if A == C {
            return Ok(q(self._v))
        }
        if A > Q_MAX_PRECISION || C > Q_MAX_PRECISION {
            return Err(QE::PrecisionTooLarge)
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
                .ok_or(QE::Overflow)?
                .checked_div(old_scale)
                .ok_or(QE::DivisionByZero)?;
            if result > B::max_value().to_i128().unwrap() {
                return Err(QE::Overflow)
            }
            if result < B::min_value().to_i128().unwrap() {
                return Err(QE::Underflow)
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
            .ok_or(QE::Overflow)?
            .checked_div(old_scale)
            .ok_or(QE::DivisionByZero)?;
        if result > B::max_value().to_u128().unwrap() {
            return Err(QE::Overflow)
        }
        k_int::<C, u128, C, B>(result)
    }
}