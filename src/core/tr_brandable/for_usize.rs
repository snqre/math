boiler::extend!();

impl Brandable for usize {
    fn brand(&self) -> Brand {
        Brand::USize
    }
}