boiler::extend!();

impl _Branded for i32 {
    fn brand(&self) -> _Brand {
        _Brand::I32
    }
}