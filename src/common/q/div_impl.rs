use super::*;

impl<const A: u8, B, C> ::core::ops::Div for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Ok(new(C::div::<A, _>(self.v, rhs.v)?))
    }
}