use super::*;

impl<const A: u8, B: PrimInt + Branded> Eq for K<A, B> {}