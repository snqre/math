#![no_std]
#![allow(unused_imports)]

mod math;
mod dependency {
    pub use core::marker::PhantomData;
    pub use core::fmt::Debug as DebugT;
    pub use core::clone::Clone as CloneT;
    pub use core::marker::Copy as CopyT;
    pub use core::ops::Add as AddT;
    pub use core::ops::Sub as SubT;
    pub use core::ops::Mul as MulT;
    pub use core::ops::Div as DivT;
    pub use core::ops::Rem as RemT;
    pub use core::cmp::PartialEq as PartialEqT;
    pub use core::cmp::Eq as EqT;
    pub use core::cmp::PartialOrd as PartialOrdT;
    pub use core::cmp::Ord as OrdT;
    pub use core::cmp::Ordering;
    pub use thiserror::Error as ErrorT;
    pub use num_traits::int::PrimInt as PrimIntT;
    #[cfg(not(feature = "ink"))]
    pub use num_traits::float::Float as FloatT;
    #[cfg(feature = "ink")]
    pub use ink::scale::Encode as EncodeT;
    #[cfg(feature = "ink")]
    pub use ink::scale::Decode as DecodeT;
    #[cfg(feature = "ink")]
    pub use ink::scale_info::TypeInfo as TypeInfoT;
    #[cfg(feature = "ink")]
    pub use ink::storage::traits::StorageLayout as StorageLayoutT;
}