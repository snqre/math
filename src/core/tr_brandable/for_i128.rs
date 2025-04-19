use super::*;

impl Brandable for i128 {
    fn brand(&self) -> Brand {
        Brand::I128
    }
}