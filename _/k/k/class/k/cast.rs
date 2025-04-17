use super::*;

impl<const A: u8, B: PrimInt + Branded> K<A, B> {
    
    pub fn cast<const C: u8>(&self) -> KResult<K<C, B>> {
        let old_decimals: u32 = _only_safe_precision(A)?.into();
        let new_decimals: u32 = _only_safe_precision(A)?.into();
        let x: &Self = &self;
        let v: B = x._v;
        let eq: bool = old_decimals == new_decimals;
        if !eq {
            if x.is_unsigned() {
                let old_scale: u128 = 10u128.pow(old_decimals);
                let new_scale: u128 = 10u128.pow(new_decimals);
                let v: u128 = v
                    .to_u128()
                    .unwrap()
                    .checked_mul(new_scale)
                    .ok_or(KError::Overflow)?
                    .checked_div(old_scale)
                    .ok_or(KError::DivisionByZero)?;

                return
            }

            return
        }



        let old_decimals: u32 = _only_safe_precision(A)?.into();
        let new_decimals: u32 = _only_safe_precision(C)?.into();
        let is_same: bool = old_decimals == new_decimals;
        let v: B = self._v;
        if !is_same {
            let signed: bool = v < B::zero();
            if !signed {
                let old_scale: u128 = 10u128.pow(old_decimals);
                let new_scale: u128 = 10u128.pow(new_decimals);
                let v: u128 = v
                    .to_u128()
                    .unwrap()
                    .checked_mul(new_scale)
                    .ok_or(KError::Overflow)?
                    .checked_div(old_scale)
                    .ok_or(KError::DivisionByZero)?;
                let max: u128 = B::max_value().to_u128().unwrap();
                let v_safe: bool = v <= max;
                if !v_safe { 
                    let e: KError = KError::Overflow;
                    Err(e)
                } else {
                    let v: B = B::from(v).unwrap();
                    Ok(k(v))
                }
            } else {
                let old_scale: i128 = 10i128.pow(old_decimals);
                let new_scale: i128 = 10i128.pow(new_decimals);
                let v: i128 = v
                    .to_i128()
                    .unwrap()
                    .checked_mul(new_scale)
                    .ok_or(KError::Overflow)?
                    .checked_div(old_scale)
                    .ok_or(KError::DivisionByZero)?;
                let max: i128 = B::max_value().to_i128().unwrap();
                let min: i128 = B::min_value().to_i128().unwrap();
                let v_le_max: bool = v <= max;
                let v_ge_min: bool = v >= min;
                if !v_le_max {
                    let e: KError = KError::Overflow;
                    Err(e)
                } else if !v_ge_min {
                    let e: KError = KError::Underflow;
                    Err(e)
                } else {
                    let v: B = B::from(v).unwrap();
                    Ok(k(v))
                }
            }
        } else {
            let v: K<C, B> = k(v);
            Ok(v)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cast() {
        let _: K<18u8, i128> = k::<2u8, i128>(-200i128).cast().unwrap();
        
    }
}