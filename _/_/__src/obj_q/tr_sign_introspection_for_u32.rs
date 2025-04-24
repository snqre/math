boiler::extend!();

impl _SignIntrospection for u32 {
    fn is_signed(&self) -> bool {
        true
    }
}