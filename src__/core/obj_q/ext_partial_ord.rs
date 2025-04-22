boiler::extend!();

impl<const A: u8, B: PrimInt> PartialOrd for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    fn gt(&self, other: &Self) -> bool {
        self._v > other._v
    }

    fn lt(&self, other: &Self) -> bool {
        self._v < other._v
    }

    fn ge(&self, other: &Self) -> bool {
        self._v >= other._v
    }

    fn le(&self, other: &Self) -> bool {
        self._v <= other._v
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}