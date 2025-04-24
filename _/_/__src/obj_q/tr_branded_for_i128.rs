boiler::extend!();

impl _Branded for i128 {
    fn brand(&self) -> _Brand {
        _Brand::I128
    }
}