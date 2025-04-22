boiler::extend!();

impl Branded for i128 {
    fn brand(&self) -> Brand {
        Brand::I128
    }
}