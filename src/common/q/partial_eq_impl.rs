use super::*;

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let x: B = self.v;
        let y: B = other.v;
        x == y
    }
}