boiler::extend!();
use num_traits::int::PrimInt;

// single entry point to build the q type.
// will panic if combinations are wrong, strongly suggested
// to use the pre built types instead and use the contrsuctor
// to instantiate them.
///
/// 
/// 
/// 
/// ```
/// let balance: Q2U8 = q(10u8);
/// ```
pub fn q<const A: u8, B: PrimInt + Brandable>(v: B) -> Q::<A, B> where _CheckPrecision<A>: _IsPrecision {
    Q::new(v)
}

impl<const A: u8, B: PrimInt + Brandable> Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    
    /// Do not use a u8 for 38, will panic!.
    pub fn new(v: B) -> Self {
        debug_assert!(matches!(A, 1u8 | 2u8 | 3u8 | 4u8 | 5u8 | 6u8 | 7u8));
        assert!(match (A, v.brand()) {
            (1u8, Brand::U8) => true,
            (1u8, Brand::U16) => true,
            (1u8, Brand::U32) => true,
            _ => false,
        }, "");
        Self {
            _v: v,
        }
    }
}