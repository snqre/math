use super::{
    Int,
    Engine,
    Q,
    Result,
    HyperbolicRatio
};

impl<const A: u8, B: Int, C: Engine> Q<A, B, C> {
    #[inline]
    pub fn tanh(&self) -> Result<HyperbolicRatio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.tanh::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }

    #[inline]
    pub fn sinh(&self) -> Result<HyperbolicRatio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.sinh::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }

    #[inline]
    pub fn cosh(&self) -> Result<HyperbolicRatio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.cosh::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }
}