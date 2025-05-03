#![no_std]
#![allow(unused)]
#![allow(unused_braces)]
#![allow(unused_import_braces)]

mod common;
pub mod math;

#[cfg(feature = "color")]
mod color;

#[cfg(feature = "coordinate")]
mod coordinate;