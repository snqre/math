use super::{
    Int,
    Engine,
    Ratio,
    Q,
    Result
};

impl<const A: u8, B: Int, C: Engine> Q<A, B, C> {
    #[inline]
    pub fn csc(&self) -> Result<Ratio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.csc::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }

    #[inline]
    pub fn sec(&self) -> Result<Ratio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.sec::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }

    #[inline]
    pub fn cot(&self) -> Result<Ratio<Self>> {
        let out: B = self.v;
        let out: B = self.engine.cot::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }
}