use crate::core::tr_brandable::*;

impl Brandable for i64 {
    fn brand(&self) -> Brand {
        Brand::I64
    }
}