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
    let x: q::Q2<u128> = q::custom(500, q::DefaultEngine);
}