boiler::extend!();

use crate::core::tr_brandable::*;
use crate::core::tr_sign_introspection::SignIntrospection;
use num_traits::int::PrimInt;

pub fn k_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Brandable + SignIntrospection>(v: B) -> QR<q::<C, D>> where 
    CheckPrecision<A>: _IsPrecision,
    CheckPrecision<C>: _IsPrecision, {
    Q::new_from_int::<A, B>(v)
}

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> q<A, B> where CheckPrecision<A>: _IsPrecision {
    pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> QR<Self> where CheckPrecision<C>: _IsPrecision {
        let v: B = B::from(v).ok_or(QE::ConversionFailure)?;
        let v: q<C, B> = q(v);
        let v: q<A, B> = v.cast()?;
        Ok(v)
    }
}