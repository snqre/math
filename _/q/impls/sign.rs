use super::{
    Int,
    Engine,
    Q
};

impl<const A: u8, B: Int, C: Engine> Q<A, B, C> {
    #[inline]
    pub fn to_negative(&self) -> Self {
        let out: B = self.v;
        let out: B = self.engine.to_negative(out);
        Self::custom(out, self.engine)
    }

    #[inline]
    pub fn to_positive(&self) -> Self {
        let out: B = self.v;
        let out: B = self.engine.to_positive(out);
        Self::custom(out, self.engine)
    }
}