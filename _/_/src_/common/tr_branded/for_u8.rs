boiler::extend!();

impl Branded for u8 {
    fn brand(&self) -> Brand {
        Brand::U8
    }
}