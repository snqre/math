use crate::ambient::universal::Universal;
use crate::ambient::int::introspection::Introspection;
use crate::ambient::int::Int;
use crate::q::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct NaiveEngine;

impl Engine for NaiveEngine {
    
    fn muldiv<'a, T>(&self, x: &'a T, y: &'a T, z: &'a T) -> Result<T>
        where
            T: Int,
            T: Introspection {
        x.checked_mul(y)
            .ok_or(Error::Overflow)?
            .checked_div(z)
            .ok_or(Error::DivByZero)
    }
}