boiler::extend!();

impl _SignIntrospection for i64 {
    fn is_signed(&self) -> bool {
        true
    }
}