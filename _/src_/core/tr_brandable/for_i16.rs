use crate::core::tr_brandable::*;

impl Brandable for i16 {
    fn brand(&self) -> Brand {
        Brand::I16
    }
}