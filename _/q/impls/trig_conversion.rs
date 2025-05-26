use super::{
    Int,
    Engine,
    Q,
    Result,
    Radian,
    Degree
};

impl<const A: u8, B: Int, C: Engine> Q<A, B, C> {
    #[inline]
    pub fn to_radian(&self) -> Result<Radian<Self>> {
        let out: B = self.v;
        let out: B = self.engine.to_radian::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }

    #[inline]
    pub fn to_degree(&self) -> Result<Degree<Self>> {
        let out: B = self.v;
        let out: B = self.engine.to_degree::<A, _>(out)?;
        Ok(Self::custom(out, self.engine))
    }
}