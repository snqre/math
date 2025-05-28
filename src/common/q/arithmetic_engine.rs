use super::*;

pub trait ArithmeticEngine where Self: MuldivEngine {
    #[inline]
    fn sqrt<T>(n: semantic_fixed::Num<T>) -> semantic_fixed::Num<T> 
    where 
        T: int::Int {
        if n == T::N0 { return T::N0 }
        let mut x_0 = n / T::N2;
        let mut x_1 = (x_0 + n / x_0) / T::N2;
        while x_1 < x_0 {
            x_0 = x_1;
            x_1 = (x_0 + n / x_0) / T::N2;
        }
        x_0
    }

    #[inline]
    fn add<T>(x: semantic_fixed::Num<T>, y: semantic_fixed::Num<T>) -> Result<semantic_fixed::Num<T>> where T: int::Int {
        x.checked_add(&y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(x: semantic_fixed::Num<T>, y: semantic_fixed::Num<T>) -> Result<semantic_fixed::Num<T>> where T: int::Int {
        x.checked_sub(&y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(x: semantic_fixed::Num<B>, y: semantic_fixed::Num<B>) -> Result<semantic_fixed::Num<B>> where B: int::Int {
        Ok(Self::muldiv(x, y, scale::get::<A, _>())?)
    }

    #[inline]
    fn div<const A: u8, B>(x: semantic_fixed::Num<B>, y: semantic_fixed::Num<B>) -> Result<semantic_fixed::Num<B>> where B: int::Int {
        let scale: u128 = scale::get::<A, u128>();
        if scale.is_power_of_two() {
            let out: B = x << scale.trailing_zeros().try_into().unwrap();
            return Ok(out)
        }
        Ok(Self::muldiv(x, scale::get::<A, _>(), y)?)
    }
}

impl<T> ArithmeticEngine for T where T: Engine {}