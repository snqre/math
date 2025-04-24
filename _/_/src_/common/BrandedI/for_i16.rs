boiler::extend!();

impl BrandedI for i16 {
    fn brand(&self) -> Brand {
        Brand::I16
    }
}