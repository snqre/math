impl<const A: u8, B, C> ops::Mul for Q<A, B, C> 
where 
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.mul(x, y)
    }
}