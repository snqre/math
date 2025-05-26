use crate::q;

/// ***Brief***
/// The `NaiveEngine` is around 20% faster than the default
/// engine, but less accurate.
#[derive(Clone)]
#[derive(Copy)]
pub struct NaiveEngine;

pub fn new() -> NaiveEngine {
    NaiveEngine
}

toga::blockset! {
    impl NaiveEngine;

    q::Engine {
        fn muldiv<'a, T>(
            &self, 
            x: &'a T, 
            y: &'a T, 
            z: &'a T) -> q::Result<T> 
        where 
            T: crate::int::Int,
            T: crate::int::Introspection {
            x.checked_mul(y)
                .ok_or(q::Error::Overflow)?
                .checked_div(z)
                .ok_or(q::Error::DivisionByZero)
        }
    }
}