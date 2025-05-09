#![no_std]
#![deny(missing_debug_implementations)]
#![allow(unused)]

mod num;
mod color;
mod point_2d;
mod point_3d;
mod trig;

pub mod prelude {
    pub use crate::num::int_i::Int as _;
    pub use crate::num::int_introspection::Introspection as _;
}