use ::num_traits as num;

macro_rules! impl_for {
    ($($size:ty)*) => {
        $(
            ::paste::paste! {
                impl Int for $size {
                    const IS_SIGNED: bool = false;
                    impl_for_const! {
                        $size
                    }
                }
            }
        )*
    };
    ($($size:ty)* |*) => {
        $(
            ::paste::paste! {
                impl Int for $size {
                    const IS_SIGNED: bool = true;
                    impl_for_const! {
                        $size
                    }
                }
            }
        )*
    };
}

macro_rules! impl_for_const {
    ($size:ty) => {
        ::paste::paste! {
            const BIT: u8 = 8;
            const MAX: Self = $size::MAX;
            const MIN: Self = $size::MIN;
            const MAX_U128: u128 = $size::MAX as u128;
            const MAX_I128: i128 = $size::MAX as i128;
            const MIN_I128: i128 = $size::MIN as i128;
            const N0: Self = 0;
            const N1: Self = 1;
            const N2: Self = 2;
        }
    };
}

pub trait Int
where
    Self: num::PrimInt,
    Self: num::WrappingAdd {
    const IS_SIGNED: bool;
    const BIT: u8;
    const MAX: Self;
    const MIN: Self;
    const MAX_U128: u128;
    const MIN_U128: u128 = 0;
    const MAX_I128: i128;
    const MIN_I128: i128;
    const N0: Self;
    const N0_U128: u128 = 0;
    const N0_I128: i128 = 0;
    const N1: Self;
    const N1_U128: u128 = 1;
    const N1_I128: i128 = 1;
    const N2: Self;
    const N2_U128: u128 = 2;
    const N2_I128: i128 = 2;
}

impl_for! { u8 u16 u32 u64 u128 }
impl_for! { i8 i16 i32 i64 i128 |* }