//! This module acts as a compile-time guard to ensure that
//! any combination of `type` and `precision` is valid.
//!
//! The check is intentionally conservative: some combinations
//! may be rejected even if they are technically valid, because
//! they are not guaranteed to work with all public endpoints
//! of the parent module.
//!
//! This provides strong compile-time safety and prevents misuse,
//! especially when working with types that have limited precision
//! or range.

macro_rules! ok {
    ($($size:ty => $precision:expr)*) => {
        pub trait Ok<const A: u8, B> {}
        $(
            impl Ok<$precision, $size> for () {}
        )*
    };
} 

ok! {
    // `0` not compatible because it is redundant.

    u8 => 1
    u8 => 2

    u16 => 1
    u16 => 2
    u16 => 3
    u16 => 4

    u32 => 1
    u32 => 2
    u32 => 3
    u32 => 4
    u32 => 5
    u32 => 6
    u32 => 7
    u32 => 8
    u32 => 9

    u64 => 1
    u64 => 2
    u64 => 3
    u64 => 4
    u64 => 5
    u64 => 6
    u64 => 7
    u64 => 8
    u64 => 9
    u64 => 10
    u64 => 11
    u64 => 12
    u64 => 13
    u64 => 14
    u64 => 15
    u64 => 16
    u64 => 17
    u64 => 18
    u64 => 19

    u128 => 1
    u128 => 2
    u128 => 3
    u128 => 4
    u128 => 5
    u128 => 6
    u128 => 7
    u128 => 8
    u128 => 9
    u128 => 10
    u128 => 11
    u128 => 12
    u128 => 13
    u128 => 14
    u128 => 15
    u128 => 16
    u128 => 17
    u128 => 18
    u128 => 19
    u128 => 20
    u128 => 21
    u128 => 22
    u128 => 23
    u128 => 24
    u128 => 25
    u128 => 26
    u128 => 27
    u128 => 28
    u128 => 29
    u128 => 30
    u128 => 31
    u128 => 32
    u128 => 33
    u128 => 34
    u128 => 35
    u128 => 36
    u128 => 37
    u128 => 38

    // `i8` not compatible with `180` which is required by `trig_conversion`.
    
    i16 => 1
    i16 => 2
    i16 => 3
    i16 => 4

    i32 => 1
    i32 => 2
    i32 => 3
    i32 => 4
    i32 => 5
    i32 => 6
    i32 => 7
    i32 => 8
    i32 => 9

    i64 => 1
    i64 => 2
    i64 => 3
    i64 => 4
    i64 => 5
    i64 => 6
    i64 => 7
    i64 => 8
    i64 => 9
    i64 => 10
    i64 => 11
    i64 => 12
    i64 => 13
    i64 => 14
    i64 => 15
    i64 => 16
    i64 => 17
    i64 => 18

    i128 => 1
    i128 => 2
    i128 => 3
    i128 => 4
    i128 => 5
    i128 => 6
    i128 => 7
    i128 => 8
    i128 => 9
    i128 => 10
    i128 => 11
    i128 => 12
    i128 => 13
    i128 => 14
    i128 => 15
    i128 => 16
    i128 => 17
    i128 => 18
    i128 => 19
    i128 => 20
    i128 => 21
    i128 => 22
    i128 => 23
    i128 => 24
    i128 => 25
    i128 => 26
    i128 => 27
    i128 => 28
    i128 => 29
    i128 => 30
    i128 => 31
    i128 => 32
    i128 => 33
    i128 => 34
    i128 => 35
    i128 => 36
    i128 => 37

    // `i128 => 38` not compatible with `pi` due to potential overflow.
}