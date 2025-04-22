boiler::extend!();

impl Brandable for i16 {
    fn brand(&self) -> Brand {
        Brand::I16
    }
}