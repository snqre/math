use super::*;

impl<const A: u8, B, C> ::core::ops::Mul for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Ok(new(C::mul::<A, _>(self.v, rhs.v)?))
    }
}