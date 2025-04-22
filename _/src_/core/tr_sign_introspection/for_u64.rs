use crate::core::tr_sign_introspection::*;

impl SignIntrospection for u64 {
    fn is_signed(&self) -> bool {
        false
    }
}