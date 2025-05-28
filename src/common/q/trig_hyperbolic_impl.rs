use super::*;

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    #[inline]
    pub fn tanh(&self) -> Result<semantic::HyperbolicRatio<Self>> {
        Ok(new(C::tanh::<A, _>(self.v)?))
    }

    #[inline]
    pub fn sinh(&self) -> Result<semantic::HyperbolicRatio<Self>> {
        Ok(new(C::sinh::<A, _>(self.v)?))
    }

    #[inline]
    pub fn cosh(&self) -> Result<semantic::HyperbolicRatio<Self>> {
        Ok(new(C::cosh::<A, _>(self.v)?))
    }
}