use crate::core::tr_brandable::*;

impl Brandable for u16 {
    fn brand(&self) -> Brand {
        Brand::U16
    }
}