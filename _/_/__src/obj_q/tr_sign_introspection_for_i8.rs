boiler::extend!();

impl _SignIntrospection for i8 {
    fn is_signed(&self) -> bool {
        true
    }
}