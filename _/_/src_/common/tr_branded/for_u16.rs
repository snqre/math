boiler::extend!();

impl Branded for u16 {
    fn brand(&self) -> Brand {
        Brand::U16
    }
}