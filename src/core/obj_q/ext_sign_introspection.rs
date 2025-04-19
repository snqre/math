use super::*;
use crate::core::tr_sign_introspection::*;
use num_traits::int::PrimInt;

impl<const A: u8, B: PrimInt + SignIntrospection> SignIntrospection for Q<A, B> {
    fn is_signed(&self) -> bool {
        self._v.is_signed()
    }
}