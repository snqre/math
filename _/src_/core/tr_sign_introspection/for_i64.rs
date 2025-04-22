use crate::core::tr_sign_introspection::*;

impl SignIntrospection for i64 {
    fn is_signed(&self) -> bool {
        true
    }
}