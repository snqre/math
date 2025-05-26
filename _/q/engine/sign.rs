use super::*;

pub trait Handler {
    #[inline]
    fn to_negative<T>(&self, n: T) -> T 
    where 
        T: num::Int {
        if n == T::ZERO {
            return T::ZERO;
        }
        T::ZERO - n
    }
    
    #[inline]
    fn to_positive<T>(&self, n: T) -> T 
    where 
        T: num::Int {
        if n == T::ZERO {
            return T::ZERO;
        }
        if n > T::ZERO {
            return n;
        }
        T::ZERO - n
    }
}

impl<T> Handler for T
where
    T: Handler
{}