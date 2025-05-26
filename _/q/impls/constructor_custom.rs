use super::{
    Int,
    Engine,
    Q
};

impl<const A: u8, B: Int, C: Engine> Q<A, B, C> {
    pub fn custom(v: B, engine: C) -> Self {
        Self {
            v,
            engine
        }
    }
}
