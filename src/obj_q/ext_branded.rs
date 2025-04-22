boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn brand(&self) -> Brand {
        self.v.brand()
    }
}