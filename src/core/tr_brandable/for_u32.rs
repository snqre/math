boiler::extend!();

impl Brandable for u32 {
    fn brand(&self) -> Brand {
        Brand::U32
    }
}