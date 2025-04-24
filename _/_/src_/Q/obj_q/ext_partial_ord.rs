boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> PartialOrd for Q<A, B> where CheckPrecision<A>: IsPrecision {
    fn gt(&self, other: &Self) -> bool {
        self.v > other.v
    }

    fn lt(&self, other: &Self) -> bool {
        self.v < other.v
    }

    fn ge(&self, other: &Self) -> bool {
        self.v >= other.v
    }

    fn le(&self, other: &Self) -> bool {
        self.v <= other.v
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}