boiler::extend!();

impl<const A: u8, B: PrimInt + Brandable> Sub for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = QResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_sub(v_1).ok_or(QError::Underflow)?;
        Ok(q(v_2))
    }
}