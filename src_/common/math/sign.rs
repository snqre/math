use super::*;

#[inline]
pub fn to_negative<T>(n: T) -> T 
where 
    T: int::Int {
    if n == T::ZERO {
        return T::ZERO;
    }
    T::ZERO - n
}

#[inline]
pub fn to_positive<T>(n: T) -> T 
where 
    T: int::Int {
    if n == T::ZERO {
        return T::ZERO;
    }
    if n > T::ZERO {
        return n;
    }
    T::ZERO - n
}