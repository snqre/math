boiler::extend!();

impl BrandedI for u64 {
    fn brand(&self) -> Brand {
        Brand::U64
    }
}