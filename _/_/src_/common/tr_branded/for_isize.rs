boiler::extend!();

impl Branded for isize {
    fn brand(&self) -> Brand {
        Brand::ISize
    }
}