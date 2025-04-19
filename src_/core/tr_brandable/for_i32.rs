use crate::core::tr_brandable::*;

impl Brandable for i32 {
    fn brand(&self) -> Brand {
        Brand::I32
    }
}