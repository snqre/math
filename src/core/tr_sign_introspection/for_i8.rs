use super::*;

impl SignIntrospection for i8 {
    fn is_signed(&self) -> bool {
        true
    }
}