use num_traits::PrimInt;

use crate::core::{tr_brandable::Brandable, tr_sign_introspection::SignIntrospection};

use super::{QE, QR};



impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Q<A, B> {
    pub fn sqrt(&self) -> QR<Self> {
        let v: B = self._v;
        if v < B::zero() {
            return Err(QE::Underflow);
        }
        if self.is_signed() {
            let v: i128 = v.to_i128().unwrap();
            let scale: u128 = 1u128 << A;
            return 
        }
        let v: u128 = v.into();
        let scale: u128 = 1u128 << A;

    }
}