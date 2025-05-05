impl<const A: u8, B, C> PartialOrd for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {

    fn ge(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v >= y._v
    }

    fn le(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v <= y._v
    }

    fn gt(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v > y._v
    }

    fn lt(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v < y._v
    }

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.cmp(other).into_some()
    }
}