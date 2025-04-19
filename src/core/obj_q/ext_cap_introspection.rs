use super::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt> Q<A, B> {
    pub fn upper_bound(&self) -> B {
        B::max_value()
    }

    pub fn lower_bound(&self) -> B {
        B::min_value()
    }
}