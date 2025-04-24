boiler::extend!();

impl _Branded for u128 {
    fn brand(&self) -> _Brand {
        _Brand::U128
    }
}