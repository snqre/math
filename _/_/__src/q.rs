use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::ops::Rem;
use core::cmp::Ordering;
use num_traits::int::PrimInt;
use thiserror::Error;

boiler::expose!(
    e,
    ext_add,
    ext_sub,
);

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B: PrimInt> where _CheckPrecision<A>: _IsPrecision {
    pub(super) _v: B,
}

mod e {
    boiler::extend!();

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

mod ext_constructor_int {
    boiler::extend!();

    pub fn q_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Branded + SignIntrospection>(v: B) -> Result<Q::<C, D>> where 
        CheckPrecision<A>: IsPrecision,
        CheckPrecision<C>: IsPrecision, {
        Q::new_from_int::<A, B>(v)
    }
    
    impl<const A: u8, B: PrimInt + Branded + SignIntrospection> Q<A, B> where CheckPrecision<A>: IsPrecision {
        pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> Result<Self> where CheckPrecision<C>: IsPrecision {
            let v: B = B::from(v).ok_or(Error::ConversionFailure)?;
            let v: Q<C, B> = q(v);
            let v: Q<A, B> = v.cast()?;
            Ok(v)
        }
    }
}

mod ext_constructor {
    boiler::extend!();

    pub fn q<const A: u8, B: PrimInt>(v: B) -> Q::<A, B> where CheckPrecision<A>: IsPrecision {
        Q::new(v)
    }
    
    impl<const A: u8, B: PrimInt> Q<A, B> where CheckPrecision<A>: IsPrecision {
        pub fn new(v: B) -> Self {
            debug_assert!(matches!(A, 1u8..=38u8));
            Self {
                v,
            }
        }
    }
}

mod ext_rem {
    boiler::extend!();

    impl<const A: u8, B: PrimInt + Branded + SignIntrospection> Rem for Q<A, B> where CheckPrecision<A>: IsPrecision {
        type Output = Result<Self>;
    
        fn rem(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            if x.is_signed() && y.is_signed() {
                let v_0: i128 = x.v.to_i128().unwrap();
                let v_1: i128 = y.v.to_i128().unwrap();
                if v_1 == 0 {
                    return Err(Error::DivisionByZero)
                }
                let v_2: i128 = v_0.checked_rem(v_1).ok_or(Error::RemByZero)?;
                let v_2: B = B::from(v_2).unwrap();
                return Ok(q(v_2))   
            }
            debug_assert!(!x.is_signed());
            debug_assert!(!y.is_signed());
            let v_0: u128 = x.v.to_u128().unwrap();
            let v_1: u128 = y.v.to_u128().unwrap();
            if v_1 == 0 {
                return Err(Error::DivisionByZero)
            }
            let v_2: u128 = v_0.checked_rem(v_1).ok_or(Error::RemByZero)?;
            let v_2: B = B::from(v_2).unwrap();
            Ok(q(v_2))
        }
    } 
}

mod ext_add {
    boiler::extend!();

    impl<const A: u8, B: PrimInt + _Branded> Add for Q<A, B> where CheckPrecision<A>: IsPrecision {
        type Output = Result<Self>;
    
        fn add(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x.v;
            let v_1: &B = &y.v;
            let v_2: B = v_0.checked_add(v_1).ok_or(Error::Overflow)?;
            Ok(q(v_2))
        }
    }
}

mod ext_sub {
    boiler::extend!();

    impl<const A: u8, B: PrimInt + Branded> Sub for Q<A, B> where CheckPrecision<A>: IsPrecision {
        type Output = Result<Self>;
    
        fn sub(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x.v;
            let v_1: &B = &y.v;
            let v_2: B = v_0.checked_sub(v_1).ok_or(Error::Underflow)?;
            Ok(q(v_2))
        }
    }
}

mod ext_mul {
    boiler::extend!();

    impl<const A: u8, B: PrimInt + Branded + SignIntrospection> Mul for Q<A, B> where CheckPrecision<A>: IsPrecision {
        type Output = Result<Self>;
    
        fn mul(self, rhs: Self) -> Self::Output {
            if A > Q_MAX_PRECISION {
                return Err(Error::PrecisionTooLarge)
            }
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x.v;
            let v_1: &B = &y.v;
            if A < Q_MIN_PRECISION {
                let v_2: B = v_0.checked_mul(v_1).ok_or(Error::Overflow)?;
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
                    .ok_or(Error::Overflow)?
                    .checked_div(scale)
                    .ok_or(Error::DivisionByZero)?;
                if v_2 > x.max_value().to_i128().unwrap() {
                    return Err(Error::Overflow)
                }
                if v_2 < x.min_value().to_i128().unwrap() {
                    return Err(Error::Underflow)
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
                .ok_or(Error::Overflow)?
                .checked_div(scale)
                .ok_or(Error::DivisionByZero)?;
            if v_2 > x.max_value().to_u128().unwrap() {
                return Err(Error::Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
            Ok(v_2)
        }
    }
}

mod ext_div {
    boiler::extend!();

    impl<const A: u8, B: PrimInt + _Branded + SignIntrospection> Div for Q<A, B> where CheckPrecision<A>: IsPrecision {
        type Output = Result<Self>;
    
        fn div(self, rhs: Self) -> Self::Output {
            if A > Q_MAX_PRECISION {
                return Err(Error::PrecisionTooLarge)
            }
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x.v;
            let v_1: &B = &y.v;
            if A < Q_MIN_PRECISION {
                let v_2: B = v_0.checked_div(v_1).ok_or(Error::DivisionByZero)?;
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
                        .ok_or(Error::DivisionByZero)?
                } else {
                    let scale: i128 = scale.try_into().unwrap();
                    v_0
                        .checked_mul(scale)
                        .ok_or(Error::Overflow)?
                        .checked_div(v_1)
                        .ok_or(Error::DivisionByZero)?
                };
                if v_2 > x.max_value().to_i128().unwrap() {
                    return Err(Error::Overflow)
                }
                if v_2 < x.min_value().to_i128().unwrap() {
                    return Err(Error::Underflow)
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
                    .ok_or(Error::DivisionByZero)?
            } else {
                v_1
                    .checked_mul(scale)
                    .ok_or(Error::Overflow)?
                    .checked_div(v_1)
                    .ok_or(Error::DivisionByZero)?
            };
            if v_2 > x.max_value().to_u128().unwrap() {
                return Err(Error::Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
            Ok(v_2)
        }
    }
}


// #region

boiler::expose!(
    tr_brand,
    tr_brand_for_i8,
    tr_brand_for_i16,
    tr_brand_for_i32,
);

mod tr_brand {
    boiler::extend!();

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub(super) enum _Brand {
        I8,
        I16,
        I32,
        I64,
        I128,
        ISize,
        U8,
        U16,
        U32,
        U64,
        U128,
        USize,
    }

    pub(super) trait _Branded: PrimInt {
        fn brand(&self) -> _Brand;
    }
}

mod tr_brand_for_i8 {
    boiler::extend!();

    impl _Branded for i8 {
        fn brand(&self) -> _Brand {
            _Brand::I8
        }
    }
}

mod tr_brand_for_i16 {
    boiler::extend!();

    impl _Branded for i16 {
        fn brand(&self) -> _Brand {
            _Brand::I16
        }
    }
}

mod tr_brand_for_i32 {
    boiler::extend!();

    impl _Branded for i32 {
        fn brand(&self) -> _Brand {
            _Brand::I32
        }
    }
}

mod tr_brand_for_i64 {
    boiler::extend!();

    impl _Branded for i64 {
        fn brand(&self) -> _Brand {
            _Brand::I64
        }
    }
}




mod tr_sign_introspection {
    pub(super) trait _SignIntrospection {
        fn is_signed(&self) -> bool;
    }
}

mod tr_sign_introspection_for_i8 {
    boiler::extend!();

    impl _SignIntrospection for i8 {
        fn is_signed(&self) -> bool {
            true
        }
    }
}


impl _Branded for i128 {
    fn brand(&self) -> _Brand {
        _Brand::I128
    }
}

impl _Branded for isize {
    fn brand(&self) -> _Brand {
        _Brand::ISize
    }
}






impl _SignIntrospection for i16 {
    fn is_signed(&self) -> bool {
        true
    }
}


pub(super) trait _IsPrecision {}