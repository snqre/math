use super::{
    Int,
    Q,
    DefaultEngine
};

impl<const A: u8, B: Int> From<B> for Q<A, B, DefaultEngine> {
    fn from(value: B) -> Self {
        Self::new(value)
    }
}