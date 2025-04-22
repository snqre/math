boiler::extend!();

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Mul for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = QResult<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if A > Q_MAX_PRECISION {
            return Err(QError::PrecisionTooLarge)
        }
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        if A < Q_MIN_PRECISION {
            let v_2: B = v_0.checked_mul(v_1).ok_or(QError::Overflow)?;
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
                .ok_or(QError::Overflow)?
                .checked_div(scale)
                .ok_or(QError::DivisionByZero)?;
            if v_2 > x.max_value().to_i128().unwrap() {
                return Err(QError::Overflow)
            }
            if v_2 < x.min_value().to_i128().unwrap() {
                return Err(QError::Underflow)
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
            .ok_or(QError::Overflow)?
            .checked_div(scale)
            .ok_or(QError::DivisionByZero)?;
        if v_2 > x.max_value().to_u128().unwrap() {
            return Err(QError::Overflow)
        }
        let v_2: B = B::from(v_2).unwrap();
        let v_2: Q<A, B> = q(v_2);
        Ok(v_2)
    }
}