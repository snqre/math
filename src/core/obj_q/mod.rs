boiler::bundle!("src/core/obj_q");
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
);

mod main {
    use thiserror::Error;
    use num_traits::int::PrimInt;

    pub const Q_MIN_PRECISION: u8 = 1u8;
    pub const Q_MAX_PRECISION: u8 = 38u8;

    pub type QR<T> = core::result::Result<T, QE>;

    #[derive(
        Debug,
        Clone,
        PartialEq,
        Error
    )]
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

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Copy)]
    pub struct Q<const A: u8, B: PrimInt> {
        pub(super) _v: B,
    }
}