use super::*;

impl SignIntrospection for i128 {
    fn is_signed(&self) -> bool {
        true
    }
}