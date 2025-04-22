boiler::extend!();

use crate::core::tr_brandable::*;
use crate::core::tr_sign_introspection::SignIntrospection;
use num_traits::int::PrimInt;

pub fn q_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Brandable + SignIntrospection>(v: B) -> QResult<Q::<C, D>> where 
    _CheckPrecision<A>: _IsPrecision,
    _CheckPrecision<C>: _IsPrecision, {
    Q::new_from_int::<A, B>(v)
}

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Q<A, B> where CheckPrecision<A>: _IsPrecision {
    pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> QResult<Self> where CheckPrecision<C>: _IsPrecision {
        let v: B = B::from(v).ok_or(QError::ConversionFailure)?;
        let v: q<C, B> = q(v);
        let v: q<A, B> = v.cast()?;
        Ok(v)
    }
}