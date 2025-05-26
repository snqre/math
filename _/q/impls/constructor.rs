use super::{
    Int,
    Q,
    DefaultEngine
};

impl<const A: u8, B: Int> Q<A, B, DefaultEngine> {
    pub fn new(v: B) -> Self {
        Self {
            v,
            engine: DefaultEngine
        }
    }
}