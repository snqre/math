boiler::extend!();

impl _SignIntrospection for u128 {
    fn is_signed(&self) -> bool {
        true
    }
}