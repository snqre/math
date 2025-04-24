boiler::extend!();

impl _Branded for u32 {
    fn brand(&self) -> _Brand {
        _Brand::U32
    }
}