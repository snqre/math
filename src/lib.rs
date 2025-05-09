#![no_std]
#![allow(non_camel_case_types)]

pub mod engine {
    pub mod default_engine {
        pub use crate::default_engine::*;
    }

    pub mod naive_engine {
        pub use crate::naive_engine::*;
    }
}

pub mod precision;
pub mod q;

mod default_engine;
mod int;
mod math;
mod naive_engine;
mod trig;

pub mod point_2d;
pub mod point_3d;
pub mod point_4d;