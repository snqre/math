boiler::extend!();

impl _Branded for i8 {
    fn brand(&self) -> _Brand {
        _Brand::I8
    }
}