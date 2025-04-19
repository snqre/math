use crate::core::tr_brandable::*;
use crate::core::tr_sign_introspection::SignIntrospection;
use super::*;
use num_traits::int::PrimInt;

pub fn k_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Brandable + SignIntrospection>(v: B) -> QR<Q::<C, D>> {
    Q::new_from_int::<A, B>(v)
}

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Q<A, B> {
    pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> QR<Self> {
        let v: B = B::from(v).ok_or(QE::ConversionFailure)?;
        let v: Q<C, B> = q(v);
        let v: Q<A, B> = v.cast()?;
        Ok(v)
    }
}