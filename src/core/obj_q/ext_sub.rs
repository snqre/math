use super::*;
use std::ops::Sub;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt> Sub for Q<A, B> {
    type Output = QR<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_sub(v_1).ok_or(QE::Underflow)?;
        Ok(q(v_2))
    }
}