::boiler::extend!();

impl<const A: u8, B, C> ops::Add for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.add(x, y)
    }
}