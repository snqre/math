boiler::extend!();

impl SignIntrospection for usize {
    fn is_signed(&self) -> bool {
        false
    }
}