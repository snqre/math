use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    #[inline]
    pub fn to_radian(&self) -> Result<Self> {
        Ok(new(C::to_radian::<A, _>(self.v)?))
    }

    #[inline]
    pub fn to_degree(&self) -> Result<Self> {
        Ok(new(C::to_degree::<A, _>(self.v)?))
    }
}