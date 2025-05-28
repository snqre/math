use super::*;

impl<const A: u8, B, C> Q<A, B, C> 
where
    B: int::Int,
    C: Engine {
    pub fn sqrt(&self) -> Self {
        new(C::sqrt(self.v))
    }
}