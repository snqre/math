boiler::extend!();

impl _SignIntrospection for i128 {
    fn is_signed(&self) -> bool {
        true
    }
}