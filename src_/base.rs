use super::*;

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
pub fn add<T>(x: semantic_fixed::Num<T>, y: semantic_fixed::Num<T>) -> Result<semantic_fixed::Num<T>>
where
    T: num::Int {
    x.checked_add(&y).ok_or(Error::Overflow)
}

#[inline]
pub fn sub<T>(x: semantic_fixed::Num<T>, y: semantic_fixed::Num<T>) -> Result<semantic_fixed::Num<T>>
where
    T: num::Int {
    x.checked_sub(&y).ok_or(Error::Underflow)
}

#[inline]
pub fn mul<const A: u8, B>(x: semantic_fixed::Num<B>, y: semantic_fixed::Num<B>) -> Result<semantic_fixed::Num<B>>
where
    B: num::Int,
    (): precision::Ok<A, B> {
    raw::muldiv(x, y, scale::get::<A, _>())
}

#[inline]
pub fn div<const A: u8, B>(x: semantic_fixed::Num<B>, y: semantic_fixed::Num<B>) -> Result<semantic_fixed::Num<B>>
where
    B: Int,
    (): Ok<A, B>,
    (): Ok<A, u128> {
    
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