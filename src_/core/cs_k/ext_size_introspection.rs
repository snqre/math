use crate::core::cs_k::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn upper_bound(&self) -> B {
        B::max_value()
    }

    pub fn lower_bound(&self) -> B {
        B::min_value()
    }

    pub fn upper_bound_space_left(&self) -> B {
        self.upper_bound().checked_sub(&self._v)
    }

    pub fn lower_bound_space_left(&self) -> B {
        // amount left to underflow
    }

    pub fn will_overflow_on_add(&self, v: B) -> bool {
        // will this overflow if this amount is added to this value
    }

    fn will_overflow_on_mul(&self, v: B) -> bool {

    }

    fn will_underflow_on_sub(&self, v: B) -> bool {
        // will this underflow if this amount is removed from this value
    }

    fn can_contain(&self, v: B) -> bool {
        // can this value safely contain this amount
    }
}