pub(super) trait IsPrecision {}

#[repr(transparent)]
pub(super) struct CheckPrecision<const A: u8>;
    
macro_rules! _for_precision {
    ($($precision:literal),*) => {
        $(impl IsPrecision for CheckPrecision<$precision> {})*
    };
}

_for_precision!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38);