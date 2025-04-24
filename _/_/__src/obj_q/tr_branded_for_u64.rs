boiler::extend!();

impl _Branded for u64 {
    fn brand(&self) -> _Brand {
        _Brand::U64
    }
}