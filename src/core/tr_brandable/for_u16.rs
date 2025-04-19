use super::*;

impl Brandable for u16 {
    fn brand(&self) -> Brand {
        Brand::U16
    }
}