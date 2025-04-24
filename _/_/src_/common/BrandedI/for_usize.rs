boiler::extend!();

impl BrandedI for usize {
    fn brand(&self) -> Brand {
        Brand::USize
    }
}