use crate::core::tr_sign_introspection::*;

impl SignIntrospection for u16 {
    fn is_signed(&self) -> bool {
        false
    }
}