use super::*;

impl Brandable for u128 {
    fn brand(&self) -> Brand {
        Brand::U128
    }
}