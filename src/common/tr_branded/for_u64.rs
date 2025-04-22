boiler::extend!();

impl Branded for u64 {
    fn brand(&self) -> Brand {
        Brand::U64
    }
}