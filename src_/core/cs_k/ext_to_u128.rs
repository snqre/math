use crate::core::cs_k::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn to_u128(&self) -> KResult<u8> {
        self._v.to_u128().ok_or(KError::ConversionFailure)
    }
}