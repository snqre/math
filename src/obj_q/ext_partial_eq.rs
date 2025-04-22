boiler::extend!();

impl<const A: u8, B: PrimInt> PartialEq for Q<A, B> where CheckPrecision<A>: IsPrecision {
    fn eq(&self, other: &Self) -> bool {
        self._v == other._v
    }
}