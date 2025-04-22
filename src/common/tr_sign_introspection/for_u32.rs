boiler::extend!();

impl SignIntrospection for u32 {
    fn is_signed(&self) -> bool {
        true
    }
}