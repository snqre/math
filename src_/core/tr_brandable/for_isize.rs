use crate::core::tr_brandable::*;

impl Brandable for isize {
    fn brand(&self) -> Brand {
        Brand::ISize
    }
}