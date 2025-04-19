use super::*;

impl Brandable for i64 {
    fn brand(&self) -> Brand {
        Brand::I64
    }
}