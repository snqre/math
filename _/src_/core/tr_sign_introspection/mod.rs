#![allow(unused_imports)]

mod for_i8;
mod for_i16;
mod for_i32;
mod for_i64;
mod for_i128;
mod for_isize;
mod for_u8;
mod for_u16;
mod for_u32;
mod for_u64;
mod for_u128;
mod for_usize;

mod core {
    pub trait SignIntrospection {
        fn is_signed(&self) -> bool;
    }
}

pub use for_i8::*;
pub use for_i16::*;
pub use for_i32::*;
pub use for_i64::*;
pub use for_i128::*;
pub use for_isize::*;
pub use for_u8::*;
pub use for_u16::*;
pub use for_u32::*;
pub use for_u64::*;
pub use for_u128::*;
pub use for_usize::*;
pub use core::*;