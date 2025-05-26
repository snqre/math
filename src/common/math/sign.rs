use super::*;

#[inline]
pub fn to_negative<T>(n: T) -> T 
where 
    T: int::Int {
    if n == T::N0 {
        return T::N0;
    }
    T::N0 - n
}

#[inline]
pub fn to_positive<T>(n: T) -> T 
where 
    T: int::Int {
    if n == T::N0 {
        return T::N0;
    }
    if n > T::N0 {
        return n;
    }
    T::N0 - n
}