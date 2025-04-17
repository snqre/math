#![deny(warnings)]
#![allow(clippy::let_with_type_underscore)]

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::k::prelude::*;

    pub use crate::k::main::*;
    pub use crate::k::ext_add;
    pub use crate::k::ext_constructor;
    pub use crate::extension::ext_i8;
    pub use crate::extension::ext_i16;
    pub use crate::extension::ext_i32;
    pub use crate::extension::ext_i64;
    pub use crate::extension::ext_i128;
    pub use crate::extension::ext_u8;
    pub use crate::extension::ext_u16;
    pub use crate::extension::ext_u32;
    pub use crate::extension::ext_u64;
    pub use crate::extension::ext_u128;
    pub use crate::extension::tr_branded::Branded;
}

mod extension;
mod color;
mod coordinate;
mod k;