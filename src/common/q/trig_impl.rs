use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    #[inline]
    pub fn tan(&self) -> Result<semantic::Ratio<Self>> {
        Ok(new(C::tan::<A, _>(self.v)?))
    }

    #[inline]
    pub fn sin(&self) -> Result<semantic::Ratio<Self>> {
        Ok(new(C::sin::<A, _>(self.v)?))
    }

    #[inline]
    pub fn cos(&self) -> Result<semantic::Ratio<Self>> {
        Ok(new(C::cos::<A, _>(self.v)?))
    }
}