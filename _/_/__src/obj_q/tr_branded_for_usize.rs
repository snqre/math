boiler::extend!();

impl _Branded for usize {
    fn brand(&self) -> _Brand {
        _Brand::USize
    }
}