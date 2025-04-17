#[allow(unused_imports)]
pub mod prelude {
    pub use crate::k::main::*;
    pub use crate::k::ext_add::*;
    pub use crate::k::ext_constructor::*;
    pub use crate::k::ext_div::*;
    pub use crate::k::ext_eq::*;
    pub use crate::k::ext_mul::*;
    pub use crate::k::ext_ord::*;
    pub use crate::k::ext_partial_eq::*;
    pub use crate::k::ext_partial_ord::*;
    pub use crate::k::ext_precision_validator::*;
    pub use crate::k::ext_sign_introspection::*;
    pub use crate::k::ext_size_introspection::*;
    pub use crate::k::ext_sub::*;
}

pub mod main {
    use thiserror::Error;
    use num_traits::int::PrimInt;

    pub const MIN_PRECISION: u8 = 1u8;
    pub const MAX_PRECISION: u8 = 38u8;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(Error)]
    pub enum Error {
        #[error("")]
        Overflow,
        #[error("")]
        Underflow,
        #[error("")]
        DivisionByZero,
        #[error("")]
        ModByZero,
        #[error("")]
        PrecisionTooSmall,
        #[error("")]
        PrecisionTooLarge,
        #[error("")]
        IncompatiblePrecision,

        // Unable to convert may be due to overflow or underflow
        #[error("")]
        ConversionFailure,
    }

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Copy)]
    pub struct K<const A: u8, B: PrimInt> {
        pub(super) _v: B,
    }
}

pub mod ext_constructor {
    use crate::k::main::*;
    use num_traits::int::PrimInt;
    
    pub fn k<const A: u8, B: PrimInt>(v: B) -> K::<A, B> {
        K::new(v)
    }
    
    impl<const A: u8, B: PrimInt> K<A, B> {
        pub fn new(v: B) -> Self {
            Self {
                _v: v,
            }
        }
    }
}

pub mod ext_constructor_ {
    use crate::k::main::K;
    use crate::k::main::Error;
    use crate::k::main::Result;
    use crate::k::ext_constructor::k;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;

    /// Where A, B are from and C, D are the result
    pub fn k_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Branded>(v: B) -> Result<K::<C, D>> {
        K::new_from_int::<A, B>(v)
    }
    
    impl<const A: u8, B: PrimInt + Branded> K<A, B> {
        pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> Result<Self> {
            let v: B = B::from(v).ok_or(Error::ConversionFailure)?;
            let v: K<C, B> = k(v);
            let v: K<A, B> = v.cast()?;
            Ok(v)
        }
    }
}

pub mod ext_precision_validator {
    use crate::k::main::*;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt> K<A, B> {
        pub(super) fn _only_safe_precision(decimals: u8) -> Result<u8> {
            if decimals > MAX_PRECISION {
                Err(Error::PrecisionTooLarge)
            } else if decimals < MIN_PRECISION {
                Err(Error::PrecisionTooSmall)
            } else {
                Ok(decimals)
            }
        }
    }
}

pub mod ext_size_introspection {
    use crate::k::main::*;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt + Branded> K<A, B> {
        pub fn upper_bound(&self) -> B {
            B::max_value()
        }

        pub fn lower_bound(&self) -> B {
            B::min_value()
        }
    }
}

pub mod ext_sign_introspection {
    use crate::k::main::*;
    use crate::k::ext_sign_validator::_Sign;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt + Branded> K<A, B> {
        pub fn is_unsigned(&self) -> bool {
            let sign: _Sign = self._sign();
            matches!(sign, _Sign::Unsigned)
        }

        pub fn is_signed(&self) -> bool {
            !self.is_unsigned()
        }

        pub(super) fn _sign(&self) -> _Sign {
            Self::_sign_of(self._v)
        }
    }
}

pub mod ext_sign_validator {
    use crate::k::main::*;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub(super) enum _Sign {
        Unsigned,
        Signed,
    }

    impl<const A: u8, B: PrimInt + Branded> K<A, B> {
        pub(super) fn _sign_of<C: PrimInt + Branded>(v: C) -> _Sign {
            let brand: &str = v.brand();
            if matches!(brand, "u8" | "u16" | "u32" | "u64" | "u128") {
                _Sign::Unsigned
            } else {
                _Sign::Signed
            }
        }
    }
}

pub mod ext_conversion_unsigned {
    use crate::k::main::*;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt> K<A, B> {
        pub fn to_u8(&self) -> Result<u8> {
            self._v.to_u8().ok_or(Error::ConversionFailure)
        }

        pub fn to_u16(&self) -> Result<u16> {
            self._v.to_u16().ok_or(Error::ConversionFailure)
        }

        pub fn to_u32(&self) -> Result<u32> {
            self._v.to_u32().ok_or(Error::ConversionFailure)
        }

        pub fn to_u64(&self) -> Result<u64> {
            self._v.to_u64().ok_or(Error::ConversionFailure)
        }

        pub fn to_u128(&self) -> Result<u128> {
            self._v.to_u128().ok_or(Error::ConversionFailure)
        }

        pub fn to_usize(&self) -> Result<usize> {
            self._v.to_usize().ok_or(Error::ConversionFailure)
        }
    }
}

pub mod ext_conversion_signed {
    use crate::k::main::*;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt> K<A, B> {
        pub fn to_i8(&self) -> Result<i8> {
            self._v.to_i8().ok_or(Error::ConversionFailure)
        }

        pub fn to_i16(&self) -> Result<i16> {
            self._v.to_i16().ok_or(Error::ConversionFailure)
        }

        pub fn to_i32(&self) -> Result<i32> {
            self._v.to_i32().ok_or(Error::ConversionFailure)
        }

        pub fn to_i64(&self) -> Result<i64> {
            self._v.to_i64().ok_or(Error::ConversionFailure)
        }

        pub fn to_i128(&self) -> Result<i128> {
            self._v.to_i128().ok_or(Error::ConversionFailure)
        }

        pub fn to_isize(&self) -> Result<isize> {
            self._v.to_isize().ok_or(Error::ConversionFailure)
        }
    }
}

pub mod ext_conversion_float {
    use crate::k::main::*;
    use num_traits::{int::PrimInt, ToPrimitive};

    impl<const A: u8, B: PrimInt> K<A, B> {
        pub fn to_f32(&self) -> Result<f32> {
            let decimals: u32 = Self::_only_safe_precision(A)?.into();
            let v: f32 = self._v.to_f32().ok_or(Error::ConversionFailure)?;
            let scale: u128 = 10u128.checked_pow(decimals).unwrap();
            let scale: f32 = scale.to_f32().ok_or(Error::ConversionFailure)?;
            let v: f32 = v / scale;
            Ok(v)
        }

        pub fn to_f64(&self) -> Result<f64> {
            let decimals: u32 = Self::_only_safe_precision(A)?.into();
            let v: f64 = self._v.to_f64().ok_or(Error::ConversionFailure)?;
            let scale: u128 = 10u128.checked_pow(decimals).unwrap();
            let scale: f64 = scale.to_f64().ok_or(Error::ConversionFailure)?;
            let v: f64 = v / scale;
            Ok(v)
        }
    }
}

pub mod ext_cast {
    use crate::k::main::MAX_PRECISION;
    use crate::k::main::K;
    use crate::k::main::Error;
    use crate::k::main::Result;
    use crate::k::ext_constructor::k;
    use crate::extension::tr_branded::Branded;
    use crate::k::ext_constructor_::k_int;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt + Branded> K<A, B> {
        pub fn cast<const C: u8>(&self) -> Result<K<C, B>> {
            if A == C {
                return Ok(k(self._v))
            }
            if A > MAX_PRECISION || C > MAX_PRECISION {
                return Err(Error::PrecisionTooLarge)
            }
            let old_decimals: u32 = A.into();
            let new_decimals: u32 = C.into();
            if self.is_signed() {
                let old_scale: i128 = 10i128.pow(old_decimals);
                let new_scale: i128 = 10i128.pow(new_decimals);
                let result: i128 = self._v
                    .to_i128()
                    .unwrap()
                    .checked_mul(new_scale)
                    .ok_or(Error::Overflow)?
                    .checked_div(old_scale)
                    .ok_or(Error::DivisionByZero)?;
                if result > B::max_value().to_i128().unwrap() {
                    return Err(Error::Overflow)
                }
                if result < B::min_value().to_i128().unwrap() {
                    return Err(Error::Underflow)
                }
                return k_int::<C, i128, C, B>(result)
            }
            debug_assert!(self.is_signed());
            let old_scale: u128 = 10u128.pow(old_decimals);
            let new_scale: u128 = 10u128.pow(new_decimals);
            let result: u128 = self._v
                .to_u128()
                .unwrap()
                .checked_mul(new_scale)
                .ok_or(Error::Overflow)?
                .checked_div(old_scale)
                .ok_or(Error::DivisionByZero)?;
            if result > B::max_value().to_u128().unwrap() {
                return Err(Error::Overflow)
            }
            k_int::<C, u128, C, B>(result)
        }
    }
}

pub mod ext_rem {
    use crate::k::main::*;
    use crate::k::ext_constructor::k;
    use crate::extension::tr_branded::Branded;
    use std::ops::Rem;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt + Branded> Rem for K<A, B> {
        type Output = Result<Self>;

        fn rem(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            if x.is_unsigned() && y.is_unsigned() {
                let v_0: u128 = x._v.to_u128().unwrap();
                let v_1: u128 = y._v.to_u128().unwrap();
                if v_1 == 0 {
                    let e: Error = Error::DivisionByZero;
                    Err(e)
                } else {
                    let v_2: u128 = v_0.checked_rem(v_1).ok_or(Error::ModByZero)?;
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            } else {
                debug_assert!(x.is_signed());
                debug_assert!(y.is_signed());
                let v_0: i128 = x._v.to_i128().unwrap();
                let v_1: i128 = y._v.to_i128().unwrap();
                if v_1 == 0 {
                    let e: Error = Error::DivisionByZero;
                    Err(e)
                } else {
                    let v_2: i128 = v_0.checked_rem(v_1).ok_or(Error::ModByZero)?;
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))   
                }
            }
        }
    }
}

pub mod ext_add {
    use crate::k::main::*;
    use crate::k::ext_constructor::k;
    use std::ops::Add;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt> Add for K<A, B> {
        type Output = Result<Self>;
    
        fn add(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_add(v_1).ok_or(Error::Overflow)?;
            Ok(k(v_2))
        }
    }
}

pub mod ext_sub {
    use crate::k::main::*;
    use crate::k::ext_constructor::k;
    use std::ops::Sub;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt> Sub for K<A, B> {
        type Output = Result<Self>;
    
        fn sub(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_sub(v_1).ok_or(Error::Underflow)?;
            Ok(k(v_2))
        }
    }
}

pub mod ext_mul {
    use crate::k::main::*;
    use crate::k::ext_constructor::k;
    use crate::extension::tr_branded::Branded;
    use std::ops::Mul;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt + Branded> Mul for K<A, B> {
        type Output = Result<Self>;

        fn mul(self, rhs: Self) -> Self::Output {
            let decimals: u32 = Self::_only_safe_precision(A)?.into();
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            if x.is_unsigned() && y.is_unsigned() {
                let v_0: u128 = v_0.to_u128().unwrap();
                let v_1: u128 = v_1.to_u128().unwrap();
                let scale: u128 = 10u128.pow(decimals);
                let v_2: u128 = v_0
                    .checked_mul(v_1)
                    .ok_or(Error::Overflow)?
                    .checked_div(scale)
                    .ok_or(Error::DivisionByZero)?;
                let v_2_max: u128 = B::max_value().to_u128().unwrap();
                let v_2_ok: bool = v_2 <= v_2_max;
                if !v_2_ok {
                    let e: Error = Error::Overflow;
                    Err(e)
                } else {
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            } else {
                debug_assert!(x.is_signed());
                debug_assert!(y.is_signed());
                let v_0: i128 = v_0.to_i128().unwrap();
                let v_1: i128 = v_1.to_i128().unwrap();
                let scale: i128 = 10i128.pow(decimals);
                let v_2: i128 = v_0
                    .checked_mul(v_1)
                    .ok_or(Error::Overflow)?
                    .checked_div(scale)
                    .ok_or(Error::DivisionByZero)?;
                let v_2_max: i128 = B::max_value().to_i128().unwrap();
                let v_2_min: i128 = B::min_value().to_i128().unwrap();
                let v_2_le_max: bool = v_2 <= v_2_max;
                let v_2_ge_max: bool = v_2 >= v_2_min;
                if !v_2_le_max {
                    let e: Error = Error::Overflow;
                    Err(e)
                } else if !v_2_ge_max {
                    let e: Error = Error::Underflow;
                    Err(e)
                } else {
                    let v_2: B = B::from(v_2).unwrap();
                    Ok(k(v_2))
                }
            }
        }
    }
}

pub mod ext_div {
    use crate::k::main::*;
    use crate::k::ext_constructor::k;
    use crate::extension::tr_branded::Branded;
    use std::ops::Div;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt + Branded> Div for K<A, B> {
        type Output = Result<Self>;

        fn div(self, rhs: Self) -> Self::Output {
            let decimals: u32 = Self::_only_safe_precision(A)?.into();
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            if x.is_unsigned() && y.is_unsigned() {
                let v_0: u128 = v_0.to_u128().unwrap();
                let v_1: u128 = v_1.to_u128().unwrap();
                let scale: u128 = 10u128.pow(decimals);
                if !scale.is_power_of_two() {
                    let v_2: u128 = v_0
                        .checked_mul(scale)
                        .ok_or(Error::Overflow)?
                        .checked_div(v_1)
                        .ok_or(Error::DivisionByZero)?;
                    let v_2_max: u128 = B::max_value().to_u128().unwrap();
                    let v_2_ok: bool = v_2 <= v_2_max;
                    if !v_2_ok {
                        let e: Error = Error::Overflow;
                        Err(e)
                    } else {
                        let v_2: B = B::from(v_2).unwrap();
                        Ok(k(v_2))
                    }
                } else {
                    let scale_shift: u32 = scale.trailing_zeros();
                    let v_2: u128 = v_0
                        .checked_shl(scale_shift)
                        .unwrap()
                        .checked_div(v_1)
                        .ok_or(Error::DivisionByZero)?;
                    let v_2_max: u128 = B::max_value().to_u128().unwrap();
                    let v_2_ok: bool = v_2 <= v_2_max;
                    if !v_2_ok {
                        let e: Error = Error::Overflow;
                        Err(e)
                    } else {
                        let v_2: B = B::from(v_2).unwrap();
                        Ok(k(v_2))
                    }
                }
            } else {
                debug_assert!(x.is_signed());
                debug_assert!(y.is_signed());
                let v_0: i128 = v_0.to_i128().unwrap();
                let v_1: i128 = v_1.to_i128().unwrap();
                let scale: u128 = 10u128.pow(decimals);
                if !scale.is_power_of_two() {
                    let scale: i128 = scale.try_into().unwrap();
                    let v_2: i128 = v_0
                        .checked_mul(scale)
                        .ok_or(Error::Overflow)?
                        .checked_div(v_1)
                        .ok_or(Error::DivisionByZero)?;
                    let v_2_max: i128 = B::max_value().to_i128().unwrap();
                    let v_2_min: i128 = B::min_value().to_i128().unwrap();
                    let v_2_le_max: bool = v_2 <= v_2_max;
                    let v_2_ge_min: bool = v_2 >= v_2_min;
                    if !v_2_le_max {
                        let e: Error = Error::Overflow;
                        Err(e)
                    } else if !v_2_ge_min {
                        let e: Error = Error::Underflow;
                        Err(e)
                    } else {
                        let v_2: B = B::from(v_2).unwrap();
                        Ok(k(v_2))
                    }
                } else {
                    let scale_shift: u32 = scale.trailing_zeros();
                    let v_2: i128 = v_0
                        .checked_shl(scale_shift)
                        .unwrap()
                        .checked_div(v_1)
                        .ok_or(Error::DivisionByZero)?;
                    let v_2_max: i128 = B::max_value().to_i128().unwrap();
                    let v_2_min: i128 = B::min_value().to_i128().unwrap();
                    let v_2_le_max: bool = v_2 <= v_2_max;
                    let v_2_ge_min: bool = v_2 >= v_2_min;
                    if !v_2_le_max {
                        let e: Error = Error::Overflow;
                        Err(e)
                    } else if !v_2_ge_min {
                        let e: Error = Error::Underflow;
                        Err(e)
                    } else {
                        let v_2: B = B::from(v_2).unwrap();
                        Ok(k(v_2))
                    }
                }
            }
        }
    }
}

pub mod ext_partial_ord {
    use crate::k::main::*;
    use std::cmp::Ordering;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt> PartialOrd for K<A, B> {
        fn gt(&self, other: &Self) -> bool {
            self._v > other._v
        }
    
        fn lt(&self, other: &Self) -> bool {
            self._v < other._v
        }
    
        fn ge(&self, other: &Self) -> bool {
            self._v >= other._v
        }
    
        fn le(&self, other: &Self) -> bool {
            self._v <= other._v
        }
    
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

pub mod ext_ord {
    use crate::k::main::*;
    use crate::k::ext_constructor::k;
    use std::cmp::Ordering;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt> Ord for K<A, B> {
        fn clamp(self, min: Self, max: Self) -> Self {
            match self.cmp(&min) {
                Ordering::Greater => max,
                Ordering::Less => min,
                Ordering::Equal => self,
            }
        }
    
        fn max(self, other: Self) -> Self {
            let x: &Self = &self;
            let y: &Self = &other;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: &B = v_0.max(v_1);
            if *v_2 == *v_0 {
                k(*v_0)
            } else {
                k(*v_1)
            }
        }
    
        fn min(self, other: Self) -> Self {
            let x: &Self = &self;
            let y: &Self = &other;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: &B = v_0.min(v_1);
            if *v_2 == *v_0 {
                k(*v_0)
            } else {
                k(*v_1)
            }
        }
    
        fn cmp(&self, other: &Self) -> Ordering {
            self._v.cmp(&other._v)
        }
    }
}

pub mod ext_partial_eq {
    use crate::k::main::*;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt> PartialEq for K<A, B> {
        fn eq(&self, other: &Self) -> bool {
            self._v == other._v
        }
    }
}

pub mod ext_eq {
    use crate::k::main::*;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt> Eq for K<A, B> {}
}

#[cfg(test)]
mod test {
    use crate::k::main::*;
    use crate::k::ext_constructor::k;

    #[test]
    fn test_u_rem() {
        let x: K<2u8, u128> = k(705u128);
        let y: K<2u8, u128> = k(200u128);
        let z: K<2u8, u128> = (x % y).unwrap();
        let z_ok: K<2u8, u128> = k(105u128);
        assert_eq!(z, z_ok)
    }

    #[test]
    fn test_i_rem() {
        let x: K<2u8, i128> = k(-705i128);
        let y: K<2u8, i128> = k(200i128);
        let z: K<2u8, i128> = (x % y).unwrap();
        let z_ok: K<2u8, i128> = k(-105i128);
        assert_eq!(z, z_ok)
    }

    #[test]
    fn test_u_mul() {
        let x: K<2u8, u128> = k(750u128);
        let y: K<2u8, u128> = k(50u128);
        let z: K<2u8, u128> = (x * y).unwrap();
        let z_ok: K<2u8, u128> = k(375u128);
        assert_eq!(z, z_ok)
    }

    #[test]
    fn test_i_mul() {
        let x: K<2u8, i128> = k(-750i128);
        let y: K<2u8, i128> = k(50i128);
        let z: K<2u8, i128> = (x * y).unwrap();
        let z_ok: K<2u8, i128> = k(-375i128);
        assert_eq!(z, z_ok)
    }

    #[test]
    fn test_u_div() {
        let x: K<2u8, u128> = k(750u128);
        let y: K<2u8, u128> = k(50u128);
        let z: K<2u8, u128> = (x / y).unwrap();
        let z_ok: K<2u8, u128> = k(1500u128);
        assert_eq!(z, z_ok)
    }

    #[test]
    fn test_i_div() {
        let x: K<2u8, i128> = k(-750i128);
        let y: K<2u8, i128> = k(50i128);
        let z: K<2u8, i128> = (x / y).unwrap();
        let z_ok: K<2u8, i128> = k(-1500i128);
        assert_eq!(z, z_ok)
    }
}