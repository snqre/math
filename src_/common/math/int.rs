use ::num_traits::{
    PrimInt,
    WrappingAdd
};

pub type Ratio<T> = T;

pub type HyperbolicRatio<T> = T;

pub type F<T> = T;

pub type Radian<T> = T;

pub type Degree<T> = T;

pub trait Int: PrimInt + WrappingAdd {
    const BITS: Self;
    const BITS_USIZE: usize;
    const IS_SIGNED: bool;
    const IS_UNSIGNED: bool = !Self::IS_SIGNED;
    const MAX: Self;
    const MIN: Self;
    const MAX_U128: u128;
    const MIN_U128: u128 = 0;
    const MAX_I128: i128;
    const MIN_I128: i128 = 0;
    const ZERO: Self;
    const ZERO_U128: u128 = 0;
    const ZERO_I128: i128 = 0;
    const ONE: Self;
    const ONE_U128: u128 = 1;
    const ONE_I128: i128 = 1;
    const TWO: Self;
    const TWO_U128: u128 = 2;
    const TWO_I128: i128 = 2;
    const TWO_USIZE: usize = 2;
    const N_64: Self;
    const N_64_U128: u128 = 64;
    const N_64_I128: i128 = 64;

    fn to_int<T: Int>(&self) -> Option<T> {
        if Self::IS_SIGNED {
            let n: i128 = self.to_i128().unwrap();
            if n > T::MAX_I128 {
                return None;
            }
            if n < T::MIN_I128 {
                return None;
            }
            let n: T = T::from(n).unwrap();
            return n.into_some()
        }
        let n: u128 = self.to_u128().unwrap();
        if n > T::MAX_U128 {
            return None;
        }
        let n: T = T::from(n).unwrap();
        n.into_some()
    }
}

impl Int for i8 {
    const BITS: Self = 8;
    const BITS_USIZE: usize = i8::BITS as usize;
    const IS_SIGNED: bool = true;
    const MAX: Self = i8::MAX;
    const MIN: Self = i8::MIN;
    const MAX_U128: u128 = i8::MAX as u128;
    const MAX_I128: i128 = i8::MAX as i128;
    const MIN_I128: i128 = i8::MIN as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for i16 {
    const BITS: Self = 16;
    const BITS_USIZE: usize = i16::BITS as usize;
    const IS_SIGNED: bool = true;
    const MAX: Self = i16::MAX;
    const MIN: Self = i16::MIN;
    const MAX_U128: u128 = i16::MAX as u128;
    const MAX_I128: i128 = i16::MAX as i128;
    const MIN_I128: i128 = i16::MIN as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for i32 {
    const BITS: Self = 32;
    const BITS_USIZE: usize = i32::BITS as usize;
    const IS_SIGNED: bool = true;
    const MAX: Self = i32::MAX;
    const MIN: Self = i32::MIN;
    const MAX_U128: u128 = i32::MAX as u128;
    const MAX_I128: i128 = i32::MAX as i128;
    const MIN_I128: i128 = i32::MIN as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for i64 {
    const BITS: Self = 64;
    const BITS_USIZE: usize = i64::BITS as usize;
    const IS_SIGNED: bool = true;
    const MAX: Self = i64::MAX;
    const MIN: Self = i64::MIN;
    const MAX_U128: u128 = i64::MAX as u128;
    const MAX_I128: i128 = i64::MAX as i128;
    const MIN_I128: i128 = i64::MIN as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for i128 {
    const BITS: Self = 128;
    const BITS_USIZE: usize = i128::BITS as usize;
    const IS_SIGNED: bool = true;
    const MAX: Self = i128::MAX;
    const MIN: Self = i128::MIN;
    const MAX_U128: u128 = i128::MAX as u128;
    const MAX_I128: i128 = i128::MAX;
    const MIN_I128: i128 = i128::MIN;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for u8 {
    const BITS: Self = 8;
    const BITS_USIZE: usize = u8::BITS as usize;
    const IS_SIGNED: bool = false;
    const MAX: Self = u8::MAX;
    const MIN: Self = u8::MIN;
    const MAX_U128: u128 = u8::MAX as u128;
    const MAX_I128: i128 = u8::MAX as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for u16 {
    const BITS: Self = 16;
    const BITS_USIZE: usize = u16::BITS as usize;
    const IS_SIGNED: bool = false;
    const MAX: Self = u16::MAX;
    const MIN: Self = u16::MIN;
    const MAX_U128: u128 = u16::MAX as u128;
    const MAX_I128: i128 = u16::MAX as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for u32 {
    const BITS: Self = 32;
    const BITS_USIZE: usize = u32::BITS as usize;
    const IS_SIGNED: bool = false;
    const MAX: Self = u32::MAX;
    const MIN: Self = u32::MIN;
    const MAX_U128: u128 = u32::MAX as u128;
    const MAX_I128: i128 = u32::MAX as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for u64 {
    const BITS: Self = 64;
    const BITS_USIZE: usize = u64::BITS as usize;
    const IS_SIGNED: bool = false;
    const MAX: Self = u64::MAX;
    const MIN: Self = u64::MIN;
    const MAX_U128: u128 = u64::MAX as u128;
    const MAX_I128: i128 = u64::MAX as i128;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}

impl Int for u128 {
    const BITS: Self = 128;
    const BITS_USIZE: usize = u128::BITS as usize;
    const IS_SIGNED: bool = false;
    const MAX: Self = u128::MAX;
    const MIN: Self = u128::MIN;
    const MAX_U128: u128 = u128::MAX;
    const MAX_I128: i128 = i128::MAX;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;
    const N_64: Self = 64;
}