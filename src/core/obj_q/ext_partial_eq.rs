use super::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt> PartialEq for Q<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self._v == other._v
    }
}