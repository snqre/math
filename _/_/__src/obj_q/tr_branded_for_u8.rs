boiler::extend!();

impl _Branded for u8 {
    fn brand(&self) -> _Brand {
        _Brand::U8
    }
}