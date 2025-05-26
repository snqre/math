use super::{ Int, Engine, Q };

impl<const A: u8, B: Int, C: Engine> Eq for Q<A, B, C> {}