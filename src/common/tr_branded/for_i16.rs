boiler::extend!();

impl Branded for i16 {
    fn brand(&self) -> Brand {
        Brand::I16
    }
}