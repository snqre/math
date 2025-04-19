use super::*;

impl SignIntrospection for u64 {
    fn is_signed(&self) -> bool {
        false
    }
}