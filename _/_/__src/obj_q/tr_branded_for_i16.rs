boiler::extend!();

impl _Branded for i16 {
    fn brand(&self) -> _Brand {
        _Brand::I16
    }
}