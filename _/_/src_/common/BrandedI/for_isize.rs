boiler::extend!();

impl BrandedI for isize {
    fn brand(&self) -> Brand {
        Brand::ISize
    }
}