boiler::extend!();

impl<const A: u8, B: PrimInt + Brandable> Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    pub fn brand(&self) -> Brand {
        self._v.brand()
    }
}