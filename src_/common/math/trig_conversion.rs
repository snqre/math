use super::*;

#[inline]
pub fn to_radian<const A: u8, B>(angle: trig::Degree<int::F<B>>) -> Result<trig::Radian<int::F<B>>> 
where 
    B: int::Int, 
    (): precision::Ok<A, B> {
    let n: u8 = 180;
    let n: B = unsafe {
        B::from(n).unwrap_unchecked()
    };
    raw::muldiv(angle, pi::into::<A, _>(), n * scale::into::<A, _>())
}

#[inline]
pub fn to_degree<const A: u8, B>(angle: trig::Radian<int::F<B>>) -> Result<trig::Degree<int::F<B>>> 
where 
    B: int::Int, 
    (): precision::Ok<A, B> {
    let n: u8 = 180;
    let n: B = unsafe {
        B::from(n).unwrap_unchecked()
    };
    raw::muldiv(angle, n * scale::into::<A, _>(), pi::into::<A, _>())
}

#[cfg(test)]
#[allow(clippy::zero_prefixed_literal)]
mod test {
    use crate::common::math::trig::{Degree, Radian};

    use super::*;

    #[rstest::rstest]
    #[case(-286_000000, -4_991640)]
    fn to_radian(#[case] angle: Degree<i128>, #[case] ok: Radian<i128>) {
        let ret: trig::Radian<_> = super::to_radian::<6, _>(angle).unwrap();
        assert_eq!(ret, ok);
    }

    #[rstest::rstest]
    #[case(-4_991640, -285_999964)]
    fn to_degree(#[case] angle: Radian<i128>, #[case] ok: Degree<i128>) {
        let ret: trig::Degree<_> = super::to_degree::<6, _>(angle).unwrap();
        assert_eq!(ret, ok);
    }
}