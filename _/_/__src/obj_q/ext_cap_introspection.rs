boiler::extend!();

impl<const A: u8, B: PrimInt + SignIntrospection> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn max_value(&self) -> B {
        B::max_value()
    }

    pub fn min_value(&self) -> B {
        B::min_value()
    }

    pub fn max_rep_value(&self) -> B {
        if A == 0u8 {
            return self.max_value()
        }
        if A > Q_MAX_PRECISION {
            return B::zero()
        }
        let decimals: u32 = A.into();
        if self.is_signed() {
            let scale: i128 = 10i128.pow(decimals);
            let max_value: i128 = self.max_value().to_i128().unwrap();
            let max_value: i128 = max_value / scale;
            let max_value: B = B::from(max_value).unwrap();
            return max_value
        }
        let scale: u128 = 10u128.pow(decimals);
        let max_value: u128 = self.max_value().to_u128().unwrap();
        let max_value: u128 = max_value / scale;
        let max_value: B = B::from(max_value).unwrap();
        return max_value
    }

    pub fn min_rep_value(&self) -> B {
        if A == 0u8 {
            return self.min_value()
        }
        if A > Q_MAX_PRECISION {
            
        }
        self.max_rep_value() - (self.max_rep_value() * 2)
    }
}