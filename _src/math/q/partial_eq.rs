impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {

    fn eq(&self, other: &Self) -> bool {
        let x: &Self = self;
        let x: &B = &x._v;
        let y: &Self = other;
        let y: &B = &y._v;
        x == y
    }
}