boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> Add for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x.v;
        let v_1: &B = &y.v;
        let v_2: B = v_0.checked_add(v_1).ok_or(Error::Overflow)?;
        Ok(q(v_2))
    }
}