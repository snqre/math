boiler::extend!();

impl Brandable for isize {
    fn brand(&self) -> Brand {
        Brand::ISize
    }
}