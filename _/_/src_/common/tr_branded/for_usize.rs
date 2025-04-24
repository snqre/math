boiler::extend!();

impl Branded for usize {
    fn brand(&self) -> Brand {
        Brand::USize
    }
}