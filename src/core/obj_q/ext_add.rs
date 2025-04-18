boiler::extend!();

impl<const A: u8, B: PrimInt + Brandable> Add for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = QR<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_add(v_1).ok_or(QE::Overflow)?;
        Ok(q(v_2))
    }
}