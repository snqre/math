use super::*;

impl SignIntrospection for i16 {
    fn is_signed(&self) -> bool {
        true
    }
}