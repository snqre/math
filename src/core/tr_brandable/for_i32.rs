boiler::extend!();

impl Brandable for i32 {
    fn brand(&self) -> Brand {
        Brand::I32
    }
}