impl<const A: u8, B, C> ops::Rem for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    B: num::CheckedRem,
    C: Engine {
    type Output = Result<Self>;

    fn rem(self, rhs: Self) -> <Self as ops::Rem>::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.rem(x, y)
    }
}