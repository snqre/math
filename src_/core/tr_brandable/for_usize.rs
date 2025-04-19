use crate::core::tr_brandable::*;

impl Brandable for usize {
    fn brand(&self) -> Brand {
        Brand::USize
    }
}