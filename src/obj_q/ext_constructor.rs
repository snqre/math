boiler::extend!();

pub fn q<const A: u8, B: PrimInt>(v: B) -> Q::<A, B> where CheckPrecision<A>: IsPrecision {
    Q::new(v)
}

impl<const A: u8, B: PrimInt> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn new(v: B) -> Self {
        debug_assert!(matches!(A, 1u8..=38u8));
        Self {
            _v: v,
        }
    }
}