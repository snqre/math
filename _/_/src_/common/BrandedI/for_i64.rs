boiler::extend!();

impl BrandedI for i64 {
    fn brand(&self) -> Brand {
        Brand::I64
    }
}