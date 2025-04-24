boiler::extend!();

impl BrandedI for i32 {
    fn brand(&self) -> Brand {
        Brand::I32
    }
}