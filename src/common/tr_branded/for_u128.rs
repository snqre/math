boiler::extend!();

impl Branded for u128 {
    fn brand(&self) -> Brand {
        Brand::U128
    }
}