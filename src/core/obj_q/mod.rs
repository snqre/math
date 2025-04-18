boiler::bundle!("src/core/obj_q");

use crate::core::tr_brandable::Brand;
use crate::core::tr_brandable::Brandable;
use crate::core::tr_sign_introspection::SignIntrospection;
use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::ops::Rem;
use core::cmp::Ordering;
use num_traits::int::PrimInt;
use thiserror::Error;
use unsafe_util::*;

/// Utilities for extending the module.
/// Are unsafe because they do not protect against invariant state.
/// There are not invariant safety guarantees while creating types
/// for these. And the invariance must be checked on the consuming
/// side.
pub mod unsafe_util {
    boiler::extend!();    

    // private to discourage creating invariant types.
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Copy)]
    pub(super) struct Q<const A: u8, B: PrimInt> where _CheckPrecision<A>: _IsPrecision {
        pub(super) _v: B,
    }

    boiler::package!(
        unsafe_variant_builder
    );
}

mod main {
    boiler::extend!();

    pub const Q_MIN_PRECISION: u8 = 1u8;
    pub const Q_MAX_PRECISION: u8 = 38u8;

    pub type QR<T> = core::result::Result<T, QE>;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(Error)]
    pub enum QE {
        #[error("")]
        Overflow,
        #[error("")]
        Underflow,
        #[error("")]
        DivisionByZero,
        #[error("")]
        RemByZero,
        #[error("")]
        PrecisionTooSmall,
        #[error("")]
        PrecisionTooLarge,
        #[error("")]
        IncompatiblePrecision,
        #[error("")]
        ConversionFailure,
    }
}





trait _IsPrecision {}

macro_rules! _for_precision {
    ($($n:literal),*) => {
        $(impl _IsPrecision for _CheckPrecision<$n> {})*
    };
}

#[repr(transparent)]
struct _CheckPrecision<const A: u8>;

_for_precision!(
    1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38
);

boiler::public!(
    main,
    ext_add,
    ext_cap_introspection,
    ext_cast,
    ext_constructor_int,
    ext_constructor,
    ext_div,
    ext_eq,
    ext_log,
    ext_mul,
    ext_muldiv,
    ext_ord,
    ext_partial_eq,
    ext_partial_ord,
    ext_prime_introspection,
    ext_rem,
    ext_sign_introspection,
    ext_sqrt,
    ext_sub,
    ext_to_f32,
    ext_to_f64,
    ext_to_i8,
    ext_to_i16,
    ext_to_i32,
    ext_to_i64,
    ext_to_i128,
    ext_to_isize,
    ext_to_u8,
    ext_to_u16,
    ext_to_u32,
    ext_to_u64,
    ext_to_u128,
    ext_to_usize,
    ext_trig_reciprocal,
    ext_trig,
    ext_trunc,
    ext_try_from_i8,
    ext_try_from_i16,
    ext_try_from_i32,
    ext_try_from_i64,
    ext_try_from_i128,
    ext_try_from_isize,
    ext_try_from_u8,
    ext_try_from_u16,
    ext_try_from_u32,
    ext_try_from_u64,
    ext_try_from_u128,
    ext_try_from_usize,
    variant,
);