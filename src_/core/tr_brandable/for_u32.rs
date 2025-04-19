use crate::core::tr_brandable::*;

impl Brandable for u32 {
    fn brand(&self) -> Brand {
        Brand::U32
    }
}