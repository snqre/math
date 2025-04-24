boiler::extend!();

impl _SignIntrospection for i32 {
    fn is_signed(&self) -> bool {
        true
    }
}