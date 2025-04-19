use crate::core::tr_sign_introspection::*;

impl SignIntrospection for isize {
    fn is_signed(&self) -> bool {
        true
    }
}