use super::*;

impl<const A: u8, B: PrimInt> Sub for K<A, B> {
    type Output = KResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_sub(v_1).ok_or(KError::Underflow)?;
        Ok(k(v_2))
    }
}