use crate::num::int;
use crate::num::int_introspection;
use crate::num::q;

#[derive(Debug)]
#[derive(Clone)]
pub struct NaiveEngine;

pub fn new() -> NaiveEngine {
    NaiveEngine
}

impl q::Engine for NaiveEngine {

    fn muldiv<'a, T>(
        &self, 
        x: &'a T, 
        y: &'a T, 
        z: &'a T) -> q::Result<T> 
    where 
        T: int::Int,
        T: int_introspection::IntIntrospection {
        x.checked_mul(y)
            .ok_or(q::Error::Overflow)?
            .checked_div(z)
            .ok_or(q::Error::DivisionByZero)
    }
}