use crate::math::q;
use crate::common::int;

/// A faster but less precise engine. Based on benchmarks
/// it is at least 20% faster than the default engine.
#[derive(Debug)]
#[derive(Clone)]
pub struct NaiveEngine;

pub fn new() -> NaiveEngine {
    NaiveEngine
}

impl q::Engine for NaiveEngine {
    
    fn muldiv<'a, T>(&self, x: &'a T, y: &'a T, z: &'a T) -> q::Result<T>
    where
        T: int::Int,
        T: int::Introspection {
        x.checked_mul(y)
            .ok_or(q::Error::Overflow)?
            .checked_div(z)
            .ok_or(q::Error::DivByZero)
    }
}