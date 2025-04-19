boiler::extend!();

impl<const A: u8, B: PrimInt> Ord for Q<A, B> {
    fn clamp(self, min: Self, max: Self) -> Self {
        if self > max {
            return max
        }
        if self < min {
            return min
        }
        self
    }

    fn max(self, other: Self) -> Self {
        let x: &Self = &self;
        let y: &Self = &other;
        let v_0: B = x._v;
        let v_1: B = y._v;
        let v_2: B = v_0.max(v_1);
        if v_2 == v_0 {
            return q(v_0)
        } 
        q(v_1)
    }

    fn min(self, other: Self) -> Self {
        let x: &Self = &self;
        let y: &Self = &other;
        let v_0: B = x._v;
        let v_1: B = y._v;
        let v_2: B = v_0.min(v_1);
        if v_2 == v_0 {
            return q(v_0)
        }
        q(v_1)    
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self._v.cmp(&other._v)
    }
}