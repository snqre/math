boiler::extend!();

impl _Branded for i64 {
    fn brand(&self) -> _Brand {
        _Brand::I64
    }
}