boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> Sub for Q<A, B> where CheckPrecision<A>: IsPrecision {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x.v;
        let v_1: &B = &y.v;
        let v_2: B = v_0.checked_sub(v_1).ok_or(Error::Underflow)?;
        Ok(q(v_2))
    }
}