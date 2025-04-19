use crate::core::tr_sign_introspection::*;

impl SignIntrospection for u8 {
    fn is_signed(&self) -> bool {
        false
    }
}