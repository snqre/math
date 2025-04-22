use crate::core::tr_sign_introspection::*;

impl SignIntrospection for usize {
    fn is_signed(&self) -> bool {
        false
    }
}