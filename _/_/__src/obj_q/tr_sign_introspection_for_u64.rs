boiler::extend!();

impl _SignIntrospection for u64 {
    fn is_signed(&self) -> bool {
        true
    }
}