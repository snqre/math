boiler::extend!();

impl SignIntrospection for i64 {
    fn is_signed(&self) -> bool {
        true
    }
}