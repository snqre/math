use crate::core::tr_sign_introspection::*;

impl SignIntrospection for u128 {
    fn is_signed(&self) -> bool {
        false
    }
}