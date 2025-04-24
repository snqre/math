boiler::extend!();

impl BrandedI for i128 {
    fn brand(&self) -> Brand {
        Brand::I128
    }
}