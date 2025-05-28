use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    #[inline]
    pub fn csc(&self) -> Result<semantic::Ratio<Self>> {
        Ok(new(C::csc::<A, _>(self.v)?))
    }

    #[inline]
    pub fn sec(&self) -> Result<semantic::Ratio<Self>> {
        Ok(new(C::sec::<A, _>(self.v)?))
    }

    #[inline]
    pub fn cot(&self) -> Result<semantic::Ratio<Self>> {
        Ok(new(C::cot::<A, _>(self.v)?))
    }
}