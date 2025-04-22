boiler::extend!();

impl<const A: u8, B: PrimInt + SignIntrospection> SignIntrospection for Q<A, B> where CheckPrecision<A>: IsPrecision {
    fn is_signed(&self) -> bool {
        self._v.is_signed()
    }
}