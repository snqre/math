use crate::core::cs_k::*;
use crate::core::tr_sign_introspection::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt + SignIntrospection> SignIntrospection for K<A, B> {
    fn is_signed(&self) -> bool {
        self._v.is_signed()
    }
}