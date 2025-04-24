boiler::extend!();

impl BrandedI for u8 {
    fn brand(&self) -> Brand {
        Brand::U8
    }
}