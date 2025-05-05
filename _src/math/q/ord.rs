impl<const A: u8, B, C> Ord for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {

    fn clamp(self, min: Self, max: Self) -> Self {
        if self > max {
            return max;
        }
        if self < min {
            return min;
        }
        self
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            return self;
        }
        other
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            return self;
        }
        other
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self > other {
            return cmp::Ordering::Greater
        }
        if self < other {
            return cmp::Ordering::Less
        }
        cmp::Ordering::Equal
    }
}