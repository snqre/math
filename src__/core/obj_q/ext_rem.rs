boiler::extend!();

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Rem for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = QResult<Self>;

    fn rem(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        if x.is_signed() && y.is_signed() {
            let v_0: i128 = x._v.to_i128().unwrap();
            let v_1: i128 = y._v.to_i128().unwrap();
            if v_1 == 0 {
                return Err(QError::DivisionByZero)
            }
            let v_2: i128 = v_0.checked_rem(v_1).ok_or(QError::RemByZero)?;
            let v_2: B = B::from(v_2).unwrap();
            return Ok(q(v_2))   
        }
        debug_assert!(!x.is_signed());
        debug_assert!(!y.is_signed());
        let v_0: u128 = x._v.to_u128().unwrap();
        let v_1: u128 = y._v.to_u128().unwrap();
        if v_1 == 0 {
            return Err(QError::DivisionByZero)
        }
        let v_2: u128 = v_0.checked_rem(v_1).ok_or(QError::RemByZero)?;
        let v_2: B = B::from(v_2).unwrap();
        Ok(q(v_2))
    }
} 