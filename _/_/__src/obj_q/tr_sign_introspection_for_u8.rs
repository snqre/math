boiler::extend!();

impl _SignIntrospection for u8 {
    fn is_signed(&self) -> bool {
        true
    }
}