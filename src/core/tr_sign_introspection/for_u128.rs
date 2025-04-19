use super::*;

impl SignIntrospection for u128 {
    fn is_signed(&self) -> bool {
        false
    }
}