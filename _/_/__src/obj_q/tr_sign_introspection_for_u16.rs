boiler::extend!();

impl _SignIntrospection for u16 {
    fn is_signed(&self) -> bool {
        true
    }
}