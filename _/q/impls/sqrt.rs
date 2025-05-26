use super::{
    Int,
    Engine,
    Q
};

impl<const A: u8, B: Int, C: Engine> Q<A, B, C> {
    #[inline]
    pub fn sqrt(&self) -> Self {
        let out: B = self.v;
        let out: B = self.engine.sqrt(out);
        Self::custom(out, self.engine)
    }
}