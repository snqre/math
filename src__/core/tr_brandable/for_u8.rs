boiler::extend!();

impl Brandable for u8 {
    fn brand(&self) -> Brand {
        Brand::U8
    }
}