boiler::extend!();

impl _Branded for u16 {
    fn brand(&self) -> _Brand {
        _Brand::U16
    }
}