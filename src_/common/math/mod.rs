use ::core::hint;
use ::versal::IntoResult as _;

use crate::common::int;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

#[inline] // muldiv n, n, scale ??
pub fn sqrt<T>(n: int::F<T>) -> int::F<T> 
where 
    T: int::Int {
    if n == T::ZERO {
        return T::ZERO;
    }
    let mut x_0 = n / T::TWO;
    let mut x_1 = (x_0 + n / x_0) / T::TWO;
    while x_1 < x_0 {
        x_0 = x_1;
        x_1 = (x_0 + n / x_0) / T::TWO;
    }
    x_0
}  

#[inline]
pub fn add<T>(x: int::F<T>, y: int::F<T>) -> Result<int::F<T>> 
where 
    T: int::Int {
    x.checked_add(&y).ok_or(Error::Overflow)
}

#[inline]
pub fn sub<T>(x: int::F<T>, y: int::F<T>) -> Result<int::F<T>> 
where 
    T: int::Int {
    x.checked_sub(&y).ok_or(Error::Underflow)
}

#[inline]
pub fn mul<const A: u8, B>(x: int::F<B>, y: int::F<B>) -> Result<int::F<B>> 
where 
    B: int::Int, 
    (): precision::Ok<A, B> {
    raw::muldiv(x, y, scale::into::<A, _>())
}

#[inline]
pub fn div<const A: u8, B>(x: int::F<B>, y: int::F<B>) -> Result<int::F<B>> 
where 
    B: int::Int, 
    (): precision::Ok<A, B>, 
    (): precision::Ok<A, u128> {
    let scale: u128 = scale::into::<A, u128>();
    if scale.is_power_of_two() {
        let ret: B = x << scale.trailing_zeros().try_into().unwrap();
        return Ok(ret)
    }
    let scale: B = unsafe {
        B::from(scale).unwrap_unchecked()
    };
    raw::muldiv(x, scale, y)
}

pub mod conversion;
pub mod pi;
pub mod precision;
pub mod raw;
pub mod scale;
pub mod sign;
pub mod trig_arc;
pub mod trig_conversion;
pub mod trig_hyperbolic;
pub mod trig_reciprocal;
pub mod trig;

#[cfg(test)]
mod test {
    use ::core::fmt;

    use super::*;

    #[::rstest::rstest]
    #[case(300, 200, 600)]
    fn mul<T>(
        #[case] x: int::F<T>,
        #[case] y: int::F<T>,
        #[case] ok: int::F<T>
    )
    where
        T: fmt::Debug,
        T: int::Int,
        (): precision::Ok<2, T> {
        let ret: T = super::mul::<2, _>(x, y).unwrap();
        assert_eq!(ret, ok);
    }

    #[::rstest::rstest]
    #[case(100, 100, 100)]
    fn div<T>(
        #[case] x: int::F<T>,
        #[case] y: int::F<T>,
        #[case] ok: int::F<T>
    )
    where
        T: fmt::Debug,
        T: int::Int,
        (): precision::Ok<2, T> {
        let ret: T = super::div::<2, _>(x, y).unwrap();
        assert_eq!(ret, ok);
    }
}