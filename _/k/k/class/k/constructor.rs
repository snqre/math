use super::*;

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn new(v: B) -> Self {
        Self {
            _v: v,
        }
    }
}