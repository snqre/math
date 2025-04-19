use crate::core::tr_brandable::*;

impl Brandable for u8 {
    fn brand(&self) -> Brand {
        Brand::U8
    }
}