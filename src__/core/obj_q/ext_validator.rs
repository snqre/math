boiler::extend!();

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    
    pub(super) fn _min_rep(&self) -> f64 {
        let brand: Brand = self.brand();
        match (brand, A) {
            (Brand::U8, 1u8) => q_int::<A, u8, A, B>(2550u8).unwrap(),
            (Brand::U8, 2u8) => q_int::<A, u8, A, B>(2u8).unwrap(),
            (Brand::U8, 3u8) => q_int::<A, u8, A, B>().unwrap(),
            
        }
    }

    pub(super) fn _is_ok(v: B) -> bool {
        let brand: Brand = v.brand();
        match (brand, A) {
            (Brand::I8, 1u8) 
            | (Brand::I8, 2u8)
            | (Brand::I8, 3u8)
            | (Brand::I8, 4u8)
            | (Brand::I8, 5u8) 
            | (Brand::I8, 6u8) 
            | (Brand::I8, 7u8)
            | (Brand::I8, 8u8)
            | (Brand::I8, 9u8)
            | (Brand::I8, 10u8)
            | (Brand::I8, 11u8)
            | (Brand::I8, 12u8)
            | (Brand::I16, 1u8)
            | (Brand::I16, 2u8)
            | (Brand::I16, 3u8)
            | (Brand::I16, 4u8)
            | (Brand::I16, 5u8)
            | (Brand::I16, 6u8)
            | (Brand::I16, 7u8)
            | (Brand::I16, 8u8)
            | (Brand::I16, 9u8)
            | (Brand::I16, 10u8)
            | (Brand::I16, 11u8)
            | (Brand::I16, 12u8) => true,
            _ => false,
        }
    }
}