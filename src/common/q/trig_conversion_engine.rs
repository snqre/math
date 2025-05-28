use super::*;

pub trait TrigConversionEngine where Self: MuldivEngine {
    #[inline]
    fn to_radian<const A: u8, B>(angle: semantic_fixed::Degree<B>) -> Result<semantic_fixed::Radian<B>> where B: int::Int {
        Ok(Self::muldiv(angle, pi::get::<A, _>(), n180::<B>() * scale::get::<A, _>())?)
    }

    #[inline]
    fn to_degree<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Degree<B>> where B: int::Int {
        Ok(Self::muldiv(angle, n180::<B>() * scale::get::<A, _>(), pi::get::<A, _>())?)
    }
}

#[inline]
fn n180<T>() -> T where T: int::Int {
    if T::IS_SIGNED {
        assert!(T::BIT != 8);
    }
    let res: u8 = 180;
    let res: T = unsafe {
        T::from(res).unwrap_unchecked()
    };
    res
}

impl<T> TrigConversionEngine for T where T: Engine {}