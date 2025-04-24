boiler::extend!();

impl Branded for i8 {
    fn brand(&self) -> Brand {
        Brand::I8
    }
}