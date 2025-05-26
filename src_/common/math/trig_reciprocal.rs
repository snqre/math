use super::*;

#[inline]
pub fn csc<const A: u8, B>(angle: trig::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
where 
    B: int::Int, 
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    inv::<A, _>(trig::sin::<A, _>(angle)?)
}

#[inline]
pub fn sec<const A: u8, B>(angle: trig::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    inv::<A, _>(trig::cos::<A, _>(angle)?)
}

#[inline]
pub fn cot<const A: u8, B>(angle: trig::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    inv::<A, _>(trig::tan::<A, _>(angle)?)
}

#[inline]
fn inv<const A: u8, B>(n: B) -> Result<B> 
where
    B: int::Int,
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    div::<A, _>(scale::into::<A, _>(), n)
}