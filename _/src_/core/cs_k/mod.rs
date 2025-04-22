mod ext_add;
mod ext_cast;
mod ext_constrcutor;
mod ext_partial_ord;

#[allow(unused_imports)]
mod main {
    pub use super::core::*;
    pub use super::ext_partial_ord::*;
} 

pub use main::*;

mod core {
    use thiserror::Error;
    use num_traits::int::PrimInt;

    pub const MIN_PRECISION: u8 = 1u8;
    pub const MAX_PRECISION: u8 = 38u8;

    pub type KResult<T> = core::result::Result<T, KError>;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(Error)]
    pub enum KError {
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

mod ext_constructor {
    use super::*;
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

mod ext_int_constructor {
    use crate::core::cs_k::*;
    use crate::core::tr_brandable::*;
    use crate::core::tr_sign_introspection::*;
    use num_traits::int::PrimInt;
    
    /// Creates a new instance of `K<A, B>` from a given integer value `v` of type `B`.
    /// This function is intended to initialize a `K<A, B>` object where `A` and `B` are compile-time constants or types.
    /// 
    /// # Type Parameters
    /// 
    /// - `A`: A compile-time constant representing the first dimension of the `K` type.
    /// - `B`: A type that implements `PrimInt` and `Brandable`. This type will be used for the integer value `v`.
    /// - `C`: A compile-time constant representing the second dimension of the `K` type, used in the `K::new_from_int` method.
    /// - `D`: A type that implements `PrimInt` and `Brandable`. This type represents the type of the input value `v` passed into the function.
    /// 
    /// # Arguments
    /// 
    /// - `v`: The integer value of type `B` to be converted and used in the creation of the `K<A, B>` object. This value will be used to initialize the new `K` object.
    /// 
    /// # Returns
    /// 
    /// Returns a `KResult<K<A, B>>`, which is a result type that wraps either a `K<A, B>` object if successful or an `Error` if an issue occurs.
    pub fn k_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Brandable + SignIntrospection>(v: B) -> KResult<K::<C, D>> {
        K::new_from_int::<A, B>(v)
    }
    
    impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> K<A, B> {
        
        /// Creates a new instance of `K<A, B>` from a given integer value `v` of type `B`.
        /// This function is intended to initialize a `K<A, B>` object where `A` and `B` are compile-time constants or types.
        /// 
        /// # Type Parameters
        /// 
        /// - `A`: A compile-time constant representing the first dimension of the `K` type.
        /// - `B`: A type that implements `PrimInt` and `Brandable`. This type will be used for the integer value `v`.
        /// - `C`: A compile-time constant representing the second dimension of the `K` type, used in the `K::new_from_int` method.
        /// - `D`: A type that implements `PrimInt` and `Brandable`. This type represents the type of the input value `v` passed into the function.
        /// 
        /// # Arguments
        /// 
        /// - `v`: The integer value of type `B` to be converted and used in the creation of the `K<A, B>` object. This value will be used to initialize the new `K` object.
        /// 
        /// # Returns
        /// 
        /// Returns a `KResult<K<A, B>>`, which is a result type that wraps either a `K<A, B>` object if successful or an `Error` if an issue occurs.
        pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> KResult<Self> {
            let v: B = B::from(v).ok_or(KError::ConversionFailure)?;
            let v: K<C, B> = k(v);
            let v: K<A, B> = v.cast()?;
            Ok(v)
        }
    }
}

mod ext_size_introspection {
    use super::*;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt> K<A, B> {
        pub fn upper_bound(&self) -> B {
            B::max_value()
        }
    
        pub fn lower_bound(&self) -> B {
            B::min_value()
        }
    }
}

mod ext_sign_introspection {
    use crate::core::cs_k::*;
    use crate::core::tr_sign_introspection::*;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt + SignIntrospection> SignIntrospection for K<A, B> {
        fn is_signed(&self) -> bool {
            self._v.is_signed()
        }
    }
}

mod ext_partial_eq {

}

mod ext_eq {

}



mod ext_ord {
    use super::*;
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

mod ext_cast {
    use crate::core::cs_k::*;
    use crate::core::tr_brandable::*;
    use crate::core::tr_sign_introspection::*;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> K<A, B> {
        pub fn cast<const C: u8>(&self) -> KResult<K<C, B>> {
            use KError::*;
            if A == C {
                return Ok(k(self._v))
            }
            if A > MAX_PRECISION || C > MAX_PRECISION {
                return Err(PrecisionTooLarge)
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
                    .ok_or(Overflow)?
                    .checked_div(old_scale)
                    .ok_or(DivisionByZero)?;
                if result > B::max_value().to_i128().unwrap() {
                    return Err(Overflow)
                }
                if result < B::min_value().to_i128().unwrap() {
                    return Err(Underflow)
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
                .ok_or(Overflow)?
                .checked_div(old_scale)
                .ok_or(DivisionByZero)?;
            if result > B::max_value().to_u128().unwrap() {
                return Err(Overflow)
            }
            k_int::<C, u128, C, B>(result)
        }
    }
}

mod ext_rem {
    use crate::k_::main::*;
    use crate::k_::ext_constructor::k;
    use crate::tr_branded::main::Branded;
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

mod ext_add {
    use super::*;
    use std::ops::Add;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt> Add for K<A, B> {
        type Output = KResult<Self>;
    
        fn add(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_add(v_1).ok_or(KError::Overflow)?;
            Ok(k(v_2))
        }
    }
}

mod ext_sub {
    use crate::core::cs_k::*;
    use std::ops::Sub;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt> Sub for K<A, B> {
        type Output = KResult<Self>;
    
        fn sub(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_sub(v_1).ok_or(KError::Underflow)?;
            Ok(k(v_2))
        }
    }
}

mod ext_mul {
    use super::*;
    use crate::core::tr_brandable::*;
    use crate::core::tr_sign_introspection::*;
    use std::ops::Mul;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Mul for K<A, B> {
        type Output = KResult<Self>;
    
        fn mul(self, rhs: Self) -> Self::Output {
            use KError::*;
            if A > MAX_PRECISION {
                return Err(PrecisionTooLarge)   
            }
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            if A < MIN_PRECISION {
                let v_2: B = v_0.checked_mul(v_1).ok_or(Overflow)?;
                let v_2: K<A, B> = k(v_2);
                return Ok(v_2)
            }
            let decimals: u32 = A.into();
            if x.is_signed() && y.is_signed() {
                let v_0: i128 = v_0.to_i128().unwrap();
                let v_1: i128 = v_1.to_i128().unwrap();
                let scale: i128 = 10i128.pow(decimals);
                let v_2: i128 = v_0
                    .checked_mul(v_1)
                    .ok_or(Overflow)?
                    .checked_div(scale)
                    .ok_or(DivisionByZero)?;
                if v_2 > x.upper_bound().to_i128().unwrap() {
                    return Err(Overflow)
                }
                if v_2 < x.lower_bound().to_i128().unwrap() {
                    return Err(Underflow)
                }
                let v_2: B = B::from(v_2).unwrap();
                let v_2: K<A, B> = k(v_2);
                return Ok(v_2)
            }
            debug_assert!(!x.is_signed());
            debug_assert!(!y.is_signed());
            let v_0: u128 = v_0.to_u128().unwrap();
            let v_1: u128 = v_1.to_u128().unwrap();
            let scale: u128 = 10u128.pow(decimals);
            let v_2: u128 = v_0
                .checked_mul(v_1)
                .ok_or(Overflow)?
                .checked_div(scale)
                .ok_or(DivisionByZero)?;
            if v_2 > x.upper_bound().to_u128().unwrap() {
                return Err(Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: K<A, B> = k(v_2);
            Ok(v_2)
        }
    }
}

mod ext_div {
    use super::*;
    use crate::core::tr_brandable::*;
    use crate::core::tr_sign_introspection::*;
    use std::ops::Div;
    use num_traits::int::PrimInt;
    
    impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Div for K<A, B> {
        type Output = KResult<Self>;
    
        fn div(self, rhs: Self) -> Self::Output {
            use KError::*;
            if A > MAX_PRECISION {
                return Err(PrecisionTooLarge)
            }
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            if A < MIN_PRECISION {
                let v_2: B = v_0.checked_div(v_1).ok_or(DivisionByZero)?;
                let v_2: K<A, B> = k(v_2);
                return Ok(v_2)
            }
            let decimals: u32 = A.into();
            let scale: u128 = 10u128.pow(decimals);
            if x.is_signed() && y.is_signed() {
                let v_0: i128 = v_0.to_i128().unwrap();
                let v_1: i128 = v_1.to_i128().unwrap();
                let v_2: i128 = if scale.is_power_of_two() {
                    let scale_shift: u32 = scale.trailing_zeros();
                    v_0
                        .checked_shl(scale_shift)
                        .unwrap()
                        .checked_div(v_1)
                        .ok_or(DivisionByZero)?
                } else {
                    let scale: i128 = scale.try_into().unwrap();
                    v_0
                        .checked_mul(scale)
                        .ok_or(Overflow)?
                        .checked_div(v_1)
                        .ok_or(DivisionByZero)?
                };
                if v_2 > x.upper_bound().to_i128().unwrap() {
                    return Err(Overflow)
                }
                if v_2 < x.lower_bound().to_i128().unwrap() {
                    return Err(Underflow)
                }
                let v_2: B = B::from(v_2).unwrap();
                let v_2: K<A, B> = k(v_2);
                return Ok(v_2)
            }
            debug_assert!(!x.is_signed());
            debug_assert!(!y.is_signed());
            let v_0: u128 = v_0.to_u128().unwrap();
            let v_1: u128 = v_1.to_u128().unwrap();
            let v_2: u128 = if scale.is_power_of_two() {
                let scale_shift: u32 = scale.trailing_zeros();
                v_0
                    .checked_shl(scale_shift)
                    .unwrap()
                    .checked_div(v_1)
                    .ok_or(DivisionByZero)?
            } else {
                v_1
                    .checked_mul(scale)
                    .ok_or(Overflow)?
                    .checked_div(v_1)
                    .ok_or(DivisionByZero)?
            };
            if v_2 > x.upper_bound().to_u128().unwrap() {
                return Err(Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: K<A, B> = k(v_2);
            Ok(v_2)
        }
    }
}

mod ext_debug {

}

mod ext_display {

}

mod ext_to_u8 {
    use crate::core::cs_k::*;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt> K<A, B> {
        pub fn to_u8(&self) -> KResult<u8> {
            self._v.to_u8().ok_or(KError::ConversionFailure)
        }
    }
}

mod ext_to_u16 {
    use crate::core::cs_k::*;
    use num_traits::int::PrimInt;

    impl<const A: u8, B: PrimInt> K<A, B> {
        pub fn to_u16(&self) -> KResult<u8> {
            self._v.to_u8().ok_or(KError::ConversionFailure)
        }
    }
}