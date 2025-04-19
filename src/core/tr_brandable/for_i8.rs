boiler::extend!();

impl Brandable for i8 {
    fn brand(&self) -> Brand {
        Brand::I8
    }
}