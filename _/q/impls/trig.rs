use super::{
    Int,
    Engine,
    Ratio,
    Q,
    Result
};

impl<const A: u8, B: Int, C: Engine> Q<A, B, C> {
    #[inline]
    pub fn tan(&self) -> Result<Ratio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.tan::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }

    #[inline]
    pub fn sin(&self) -> Result<Ratio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.sin::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }
    
    #[inline]
    pub fn cos(&self) -> Result<Ratio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.cos::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }
}