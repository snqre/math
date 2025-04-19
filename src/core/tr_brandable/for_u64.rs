use super::*;

impl Brandable for u64 {
    fn brand(&self) -> Brand {
        Brand::U64
    }
}