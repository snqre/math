pub(super) trait _IsPrecision {}

#[macro_export]
macro_rules! _for_precision {
    ($($n:literal),*) => {
        $(impl _IsPrecision for _CheckPrecision<$n> {})*
    };
}

#[repr(transparent)]
pub(super) struct _CheckPrecision<const A: u8>;

_for_precision!(
    1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38
);