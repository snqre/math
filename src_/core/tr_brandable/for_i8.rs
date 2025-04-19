use crate::core::tr_brandable::*;

impl Brandable for i8 {
    fn brand(&self) -> Brand {
        Brand::I8
    }
}