use crate::core::tr_sign_introspection::*;

impl SignIntrospection for u32 {
    fn is_signed(&self) -> bool {
        false
    }
}