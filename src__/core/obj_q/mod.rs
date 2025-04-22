
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
use unsecure::*;
use variant::*;
use variant::more::*;
use variant::unsecure::*;
use precision_validation::*;

mod main {
    boiler::extend!();

    pub const Q_MIN_PRECISION: u8 = 1u8;
    pub const Q_MAX_PRECISION: u8 = 38u8;
    
    pub type QResult<T> = core::result::Result<T, QError>;
    
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(Error)]
    pub enum QError {
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
    
    // private to discourage creating invariant types.
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Copy)]
    pub struct Q<const A: u8, B: PrimInt> where _CheckPrecision<A>: _IsPrecision {
        pub(super) _v: B,
    }

    pub mod variant {
        boiler::extend!();

        /// ***Range*** `0.00E+00` `0` to `2.55E+01` `~25`
        pub type Q1U8 = Q1<u8>;

        /// ***Range*** `0.00E+00` `0` to `2.55E+00` `~2`
        pub type Q2U8 = Q2<u8>;

        /// Additiona less commonly used variants
        pub mod more {

        }

        pub mod unsecure {
            boiler::extend!();
    
            pub type Q1<T> = Q<1u8, T>;
        
            pub type Q2<T> = Q<2u8, T>;
            
            pub type Q3<T> = Q<3u8, T>;
            
            pub type Q4<T> = Q<4u8, T>;
            
            pub type Q5<T> = Q<5u8, T>;
            
            pub type Q6<T> = Q<6u8, T>;
            
            pub type Q7<T> = Q<7u8, T>;
            
            pub type Q8<T> = Q<8u8, T>;
            
            pub type Q9<T> = Q<9u8, T>;
            
            pub type Q10<T> = Q<10u8, T>;
            
            pub type Q11<T> = Q<11u8, T>;
            
            pub type Q12<T> = Q<12u8, T>;
            
            pub type Q13<T> = Q<13u8, T>;
            
            pub type Q14<T> = Q<14u8, T>;
            
            pub type Q15<T> = Q<15u8, T>;
            
            pub type Q16<T> = Q<16u8, T>;
            
            pub type Q17<T> = Q<17u8, T>;
            
            pub type Q18<T> = Q<18u8, T>;
            
            pub type Q19<T> = Q<19u8, T>;
            
            pub type Q20<T> = Q<20u8, T>;
            
            pub type Q21<T> = Q<21u8, T>;
            
            pub type Q22<T> = Q<22u8, T>;
            
            pub type Q23<T> = Q<23u8, T>;
            
            pub type Q24<T> = Q<24u8, T>;
            
            pub type Q25<T> = Q<25u8, T>;
            
            pub type Q26<T> = Q<26u8, T>;
            
            pub type Q27<T> = Q<27u8, T>;
            
            pub type Q28<T> = Q<28u8, T>;
            
            pub type Q29<T> = Q<29u8, T>;
            
            pub type Q30<T> = Q<30u8, T>;
            
            pub type Q31<T> = Q<31u8, T>;
            
            pub type Q32<T> = Q<32u8, T>;
            
            pub type Q33<T> = Q<33u8, T>;
            
            pub type Q34<T> = Q<34u8, T>;
            
            pub type Q35<T> = Q<35u8, T>;
            
            pub type Q36<T> = Q<36u8, T>;
            
            pub type Q37<T> = Q<37u8, T>;
            
            pub type Q38<T> = Q<38u8, T>;
        }
    }

    pub(super) mod precision_validation {
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
    }
}

pub fn q_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Brandable + SignIntrospection>(v: B) -> QResult<Q::<C, D>> where 
    _CheckPrecision<A>: _IsPrecision,
    _CheckPrecision<C>: _IsPrecision, {
    Q::new_from_int::<A, B>(v)
}

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Q<A, B> where CheckPrecision<A>: _IsPrecision {
pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> QResult<Self> where CheckPrecision<C>: _IsPrecision {
    let v: B = B::from(v).ok_or(QError::ConversionFailure)?;
    let v: q<C, B> = q(v);
    let v: q<A, B> = v.cast()?;
    Ok(v)
}
}

// single entry point to build the q type.
// will panic if combinations are wrong, strongly suggested
// to use the pre built types instead and use the contrsuctor
// to instantiate them.
///
/// 
/// 
/// 
/// ```
/// let balance: Q2U8 = q(10u8);
/// ```
pub fn q<const A: u8, B: PrimInt + Brandable>(v: B) -> Q::<A, B> where _CheckPrecision<A>: _IsPrecision {
    Q::new(v)
}

impl<const A: u8, B: PrimInt + Brandable> Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    
    /// Do not use a u8 for 38, will panic!.
    pub fn new(v: B) -> Self {
        debug_assert!(matches!(A, 1u8 | 2u8 | 3u8 | 4u8 | 5u8 | 6u8 | 7u8));
        assert!(match (A, v.brand()) {
            (1u8, Brand::U8) => true,
            (1u8, Brand::U16) => true,
            (1u8, Brand::U32) => true,
            _ => false,
        }, "");
        Self {
            _v: v,
        }
    }
}

impl<const A: u8, B: PrimInt + Brandable> Add for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = QResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = v_0.checked_add(v_1).ok_or(QError::Overflow)?;
        Ok(q(v_2))
    }
}

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Mul for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = QResult<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if A > Q_MAX_PRECISION {
            return Err(QError::PrecisionTooLarge)
        }
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        if A < Q_MIN_PRECISION {
            let v_2: B = v_0.checked_mul(v_1).ok_or(QError::Overflow)?;
            let v_2: Q<A, B> = q(v_2);
            return Ok(v_2)
        }
        let decimals: u32 = A.into();
        if x.is_signed() && y.is_signed() {
            let v_0: i128 = v_0.to_i128().unwrap();
            let v_1: i128 = v_1.to_i128().unwrap();
            let scale: i128 = 10i128.pow(decimals);
            let v_2: i128 = v_0
                .checked_mul(v_1)
                .ok_or(QError::Overflow)?
                .checked_div(scale)
                .ok_or(QError::DivisionByZero)?;
            if v_2 > x.max_value().to_i128().unwrap() {
                return Err(QError::Overflow)
            }
            if v_2 < x.min_value().to_i128().unwrap() {
                return Err(QError::Underflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
            return Ok(v_2)
        }
        debug_assert!(!x.is_signed());
        debug_assert!(!y.is_signed());
        let v_0: u128 = v_0.to_u128().unwrap();
        let v_1: u128 = v_1.to_u128().unwrap();
        let scale: u128 = 10u128.pow(decimals);
        let v_2: u128 = v_0
            .checked_mul(v_1)
            .ok_or(QError::Overflow)?
            .checked_div(scale)
            .ok_or(QError::DivisionByZero)?;
        if v_2 > x.max_value().to_u128().unwrap() {
            return Err(QError::Overflow)
        }
        let v_2: B = B::from(v_2).unwrap();
        let v_2: Q<A, B> = q(v_2);
        Ok(v_2)
    }
}

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> Div for Q<A, B> where _CheckPrecision<A>: _IsPrecision {
    type Output = QResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        if A > Q_MAX_PRECISION {
            return Err(QError::PrecisionTooLarge)
        }
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        if A < Q_MIN_PRECISION {
            let v_2: B = v_0.checked_div(v_1).ok_or(QError::DivisionByZero)?;
            let v_2: Q<A, B> = q(v_2);
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
                    .ok_or(QError::DivisionByZero)?
            } else {
                let scale: i128 = scale.try_into().unwrap();
                v_0
                    .checked_mul(scale)
                    .ok_or(QError::Overflow)?
                    .checked_div(v_1)
                    .ok_or(QError::DivisionByZero)?
            };
            if v_2 > x.max_value().to_i128().unwrap() {
                return Err(QError::Overflow)
            }
            if v_2 < x.min_value().to_i128().unwrap() {
                return Err(QError::Underflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
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
                .ok_or(QError::DivisionByZero)?
        } else {
            v_1
                .checked_mul(scale)
                .ok_or(QError::Overflow)?
                .checked_div(v_1)
                .ok_or(QError::DivisionByZero)?
        };
        if v_2 > x.max_value().to_u128().unwrap() {
            return Err(QError::Overflow)
        }
        let v_2: B = B::from(v_2).unwrap();
        let v_2: Q<A, B> = q(v_2);
        Ok(v_2)
    }
}

boiler::public!(
    main,
    ext_add,
    ext_brandable,
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
    ext_validator,
);