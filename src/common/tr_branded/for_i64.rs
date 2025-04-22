boiler::extend!();

impl Branded for i64 {
    fn brand(&self) -> Brand {
        Brand::I64
    }
}