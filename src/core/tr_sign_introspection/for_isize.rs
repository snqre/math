boiler::extend!();

impl SignIntrospection for isize {
    fn is_signed(&self) -> bool {
        true
    }
}