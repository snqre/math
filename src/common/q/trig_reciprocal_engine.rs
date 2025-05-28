use super::*;

pub trait TrigReciprocalEngine where Self: TrigEngine {
    #[inline]
    fn csc<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> where B: int::Int {
        Ok(Self::div::<A, _>(scale::get::<A, _>(), Self::sin::<A, _>(angle)?)?)
    }

    #[inline]
    fn sec<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> where B: int::Int {
        Ok(Self::div::<A, _>(scale::get::<A, _>(), Self::cos::<A, _>(angle)?)?)
    }

    #[inline]
    fn cot<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> where B: int::Int {
        Ok(Self::div::<A, _>(scale::get::<A, _>(), Self::tan::<A, _>(angle)?)?)
    }
}

impl<T> TrigReciprocalEngine for T where T: Engine {}