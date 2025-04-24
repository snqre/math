boiler::extend!();

impl BrandedI for i8 {
    fn brand(&self) -> Brand {
        Brand::I8
    }
}