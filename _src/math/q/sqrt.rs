::boiler::extend!();

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    
    pub fn sqrt(&self) -> Result<Self> {
        self._engine.sqrt(self)
    }
}