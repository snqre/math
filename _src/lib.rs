#![no_std]

mod common;
pub mod math;

#[cfg(feature = "color")]
mod color;

#[cfg(feature = "coordinate")]
mod coordinate;

mod direction_label;
mod direction;

mod point_2d_i;

mod q;