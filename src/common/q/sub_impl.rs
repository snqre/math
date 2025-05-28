use super::*;

impl<const A: u8, B, C> ::core::ops::Sub for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(new(C::sub(self.v, rhs.v)?))
    }
}