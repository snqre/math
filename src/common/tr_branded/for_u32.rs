boiler::extend!();

impl Branded for u32 {
    fn brand(&self) -> Brand {
        Brand::U32
    }
}