use super::*;

impl<const A: u8, B: PrimInt> TryInto<f64> for K<A, B> {
    type Error = KError;

    fn try_into(self) -> Result<f64, Self::Error> {
        let decimals: u32 = A.into();
        let scale: u128 = 10u128
            .checked_pow(decimals)
            .ok_or(KError::Overflow)
            .unwrap();
        let scale: f64 = scale as f64;
        let v: f64 = self._v.to_f64().unwrap();
        v / scale
    }
}