use super::*;

pub mod coef;
pub mod q;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

fn t() {
    let count: q::Q6<u128> = 500_000000.into();
}