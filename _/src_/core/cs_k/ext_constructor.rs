use crate::core::cs_k::*;
use num_traits::int::PrimInt;

pub fn k<const A: u8, B: PrimInt>(v: B) -> K::<A, B> {
    K::new(v)
}

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn new(v: B) -> Self {
        Self {
            _v: v,
        }
    }
}