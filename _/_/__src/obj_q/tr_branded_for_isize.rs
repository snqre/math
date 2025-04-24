boiler::extend!();

impl _Branded for isize {
    fn brand(&self) -> _Brand {
        _Brand::ISize
    }
}