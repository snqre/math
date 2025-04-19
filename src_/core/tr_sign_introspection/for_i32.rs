use crate::core::tr_sign_introspection::*;

impl SignIntrospection for i32 {
    fn is_signed(&self) -> bool {
        true
    }
}