boiler::extend!();

impl Branded for i32 {
    fn brand(&self) -> Brand {
        Brand::I32
    }
}