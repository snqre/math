boiler::extend!();

impl _SignIntrospection for usize {
    fn is_signed(&self) -> bool {
        true
    }
}