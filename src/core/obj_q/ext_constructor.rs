boiler::extend!();
use num_traits::int::PrimInt;

pub fn q<const A: u8, B: PrimInt>(v: B) -> Q::<A, B> {
    Q::new(v)
}

impl<const A: u8, B: PrimInt> Q<A, B> {
    pub fn new(v: B) -> Self {
        Self {
            _v: v,
        }
    }
}