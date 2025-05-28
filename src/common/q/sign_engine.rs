use super::*;

pub trait SignEngine {
    #[inline]
    fn to_negative<T>(n: T) -> T where T: int::Int {
        if n == T::N0 {
            return n
        } 
        T::N0 - n
    }
    
    #[inline]
    fn to_positive<T>(n: T) -> T where T: int::Int {
        if n >= T::N0 {
            return n
        } 
        T::N0 - n
    }
}

impl<T> SignEngine for T where T: Engine {}