use super::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt> Eq for Q<A, B> {}