use super::*;

impl<const A: u8, B: PrimInt> PartialEq for K<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self._v == other._v
    }
}