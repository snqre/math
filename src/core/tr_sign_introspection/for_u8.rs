boiler::extend!();

impl SignIntrospection for u8 {
    fn is_signed(&self) -> bool {
        false
    }
}