#![no_std]
#![allow(clippy::redundant_field_names)]

pub use num::Int;
pub use precision::Ok;
pub use base::sqrt;
pub use base::add;
pub use base::sub;
pub use base::mul;
pub use base::div;
pub use sign::to_negative;
pub use sign::to_positive;
pub use raw::muldiv;

mod base;
mod coef;
mod num;
mod pi;
mod precision;
mod raw;
mod scale;
mod semantic_fixed;
mod semantic;
mod sign;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}
