boiler::extend!();

impl _SignIntrospection for isize {
    fn is_signed(&self) -> bool {
        true
    }
}