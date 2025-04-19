use super::*;

impl SignIntrospection for u16 {
    fn is_signed(&self) -> bool {
        false
    }
}