use super::*;

impl<const A: u8, B: PrimInt> Div for K<A, B> {
    type Output = KResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let decimals: u32 = _only_safe_precision(A)?.into();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: B = x._v;
        let v_1: B = y._v;
        if x.is_signed() && y.is_signed() {

        }

        let signed: bool = x.is_signed() && y.is_signed();
        if !signed {
            let v_0: u128 = v_0.to_u128().unwrap();
            let v_1: u128 = v_1.to_u128().unwrap();
            let scale: u128 = 10u128.pow(decimals);
            if !scale.is_power_of_two() {
                let v_2: u128 = _muldiv(v_0, scale, v_1)?;
                let max: u128 = B::max_value().to_u128().unwrap();
                let v_2_safe: bool = v_2 <= max;
                if !v_2_safe {
                    let e: KError = KError::Overflow;
                    Err(e)
                } else {
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            } else {
                let scale_shift: u32 = scale.trailing_zeros();
                let v_2: u128 = v_0
                    .checked_shl(scale_shift)
                    .unwrap()
                    .checked_div(v_1)
                    .ok_or(KError::DivisionByZero)?;
                let max: u128 = B::max_value().to_u128().unwrap();
                let v_2_safe: bool = v_2 <= max;
                if !v_2_safe {
                    let e: KError = KError::Overflow;
                    Err(e)
                } else {
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            }
        } else {
            let v_0: i128 = v_0.to_i128().unwrap();
            let v_1: i128 = v_1.to_i128().unwrap();
            let scale: u128 = 10u128.pow(decimals);
            if !scale.is_power_of_two() {
                let scale: i128 = scale.try_into() .unwrap();
                let v_2: i128 = v_0
                    .checked_mul(scale)
                    .ok_or(KError::Overflow)?
                    .checked_div(v_1)
                    .ok_or(KError::DivisionByZero)?;
                let max: i128 = B::max_value().to_i128().unwrap();
                let min: i128 = B::min_value().to_i128().unwrap();
                let v_2_le_max: bool = v_2 <= max;
                let v_2_ge_min: bool = v_2 >= min;
                if !v_2_le_max {
                    let e: KError = KError::Overflow;
                    Err(e)
                } else if !v_2_ge_min {
                    let e: KError = KError::Underflow;
                    Err(e)
                } else {
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            } else {
                let scale_shift: u32 = scale.trailing_zeros();
                let v_2: i128 = v_0
                    .checked_shl(scale_shift)
                    .unwrap()
                    .checked_div(v_1)
                    .ok_or(KError::DivisionByZero)?;
                let max: i128 = B::max_value().to_i128().unwrap();
                let min: i128 = B::min_value().to_i128().unwrap();
                let v_2_le_max: bool = v_2 <= max;
                let v_2_ge_min: bool = v_2 >= min;
                if !v_2_le_max {
                    let e: KError = KError::Overflow;
                    Err(e)
                } else if !v_2_ge_min {
                    let e: KError = KError::Underflow;
                    Err(e)
                } else {
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            }
        }
    }
}