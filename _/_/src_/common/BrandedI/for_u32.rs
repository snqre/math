boiler::extend!();

impl BrandedI for u32 {
    fn brand(&self) -> Brand {
        Brand::U32
    }
}