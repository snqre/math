use super::*;

pub mod coef;
pub mod fold;
pub mod pi;
pub mod precision;
pub mod q;
pub mod scale;
pub mod sign;
pub mod wide_mul;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}