boiler::extend!();

impl BrandedI for u128 {
    fn brand(&self) -> Brand {
        Brand::U128
    }
}