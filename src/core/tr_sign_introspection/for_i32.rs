boiler::extend!();

impl SignIntrospection for i32 {
    fn is_signed(&self) -> bool {
        true
    }
}