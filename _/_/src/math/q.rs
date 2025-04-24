boiler::expose!(
    constructor,
    q,
    display,
    sign_introspection,
);

use crate::math::i_q::QI;
use crate::math::i_q::QIR;
use crate::math::i_q::QIE;
use crate::math::i_branded::TrBranded;
use crate::math::i_branded::BrandedIBrand;
use crate::math::i_sign_introspection::SignIntrospectionI;
use crate::math::i_precision::PrecisionI;
use crate::math::precision_check::PrecisionCheck;
use core::fmt::Debug as DebugI;
use core::fmt::Display as DisplayI;
use core::ops::Add as AddI;
use core::ops::Sub as SubI;
use core::ops::Mul as MulI;
use core::ops::Div as DivI;
use core::ops::Rem as RemI;
use core::cmp::Ordering;
use num_traits::int::PrimInt as PrimIntI;
use thiserror::Error;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B: PrimIntI> where PrecisionCheck<A>: PrecisionI {
    pub(super) _v: B,
}

mod constructor_int {
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

mod constructor {
    boiler::extend!();
    
    impl<const A: u8, B: PrimIntI> Q<A, B> where PrecisionCheck<A>: PrecisionI {
        pub fn new(v: B) -> Self {
            debug_assert!(matches!(A, 1u8..=38u8));
            Self {
                _v: v,
            }
        }
    }

    pub fn q<const A: u8, B: PrimIntI>(v: B) -> Q::<A, B> where PrecisionCheck<A>: PrecisionI {
        Q::new(v)
    }
}

mod q {
    boiler::extend!();
    
    impl<const A: u8, B: PrimIntI> QI<A, B> for Q<A, B> where PrecisionCheck<A>: PrecisionI {}
}

mod display {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI> DisplayI for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "")
        }
    }
}

mod sign_introspection {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI + SignIntrospectionI> SignIntrospectionI for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        fn is_signed(&self) -> bool {
            self._v.is_signed()
        }
    }
}

mod rem {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI + TrBranded + SignIntrospectionI> RemI for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        type Output = QIR<Self>;
    
        fn rem(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            if x.is_signed() && y.is_signed() {
                let v_0: i128 = x._v.to_i128().unwrap();
                let v_1: i128 = y._v.to_i128().unwrap();
                if v_1 == 0i128 {
                    return Err(QIE::DivisionByZero)
                }
                let v_2: i128 = v_0.checked_rem(v_1).ok_or(QIE::RemByZero)?;
                let v_2: B = B::from(v_2).unwrap();
                return Ok(q(v_2))   
            }
            debug_assert!(!x.is_signed());
            debug_assert!(!y.is_signed());
            let v_0: u128 = x._v.to_u128().unwrap();
            let v_1: u128 = y._v.to_u128().unwrap();
            if v_1 == 0u128 {
                return Err(QIE::DivisionByZero)
            }
            let v_2: u128 = v_0.checked_rem(v_1).ok_or(QIE::RemByZero)?;
            let v_2: B = B::from(v_2).unwrap();
            Ok(q(v_2))
        }
    }
}

mod add {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI + TrBranded> AddI for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        type Output = QIR<Self>;
    
        fn add(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_add(v_1).ok_or(QIE::Overflow)?;
            Ok(q(v_2))
        }
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn _test() {
            
        }
    }
}

mod sub {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI + TrBranded> SubI for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        type Output = QIR<Self>;
    
        fn sub(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_sub(v_1).ok_or(QIE::Underflow)?;
            Ok(q(v_2))
        }
    }
}

mod mul {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI + TrBranded + SignIntrospectionI> MulI for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        type Output = QIR<Self>;
    
        fn mul(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let decimals: u32 = A.into();
            if x.is_signed() && y.is_signed() {
                let v_0: i128 = v_0.to_i128().unwrap();
                let v_1: i128 = v_1.to_i128().unwrap();
                let scale: i128 = 10i128.pow(decimals);
                let v_2: i128 = v_0
                    .checked_mul(v_1)
                    .ok_or(QIE::Overflow)?
                    .checked_div(scale)
                    .ok_or(QIE::DivisionByZero)?;
                if v_2 > x.max_value().to_i128().unwrap() {
                    return Err(QIE::Overflow)
                }
                if v_2 < x.min_value().to_i128().unwrap() {
                    return Err(QIE::Underflow)
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
                .ok_or(QIE::Overflow)?
                .checked_div(scale)
                .ok_or(QIE::DivisionByZero)?;
            if v_2 > x.max_value().to_u128().unwrap() {
                return Err(QIE::Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
            Ok(v_2)
        }
    }
}

mod div {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI + TrBranded + SignIntrospectionI> DivI for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        type Output = QIR<Self>;
    
        fn div(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
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
                        .ok_or(QIE::DivisionByZero)?
                } else {
                    let scale: i128 = scale.try_into().unwrap();
                    v_0
                        .checked_mul(scale)
                        .ok_or(QIE::Overflow)?
                        .checked_div(v_1)
                        .ok_or(QIE::DivisionByZero)?
                };
                if v_2 > x.max_value().to_i128().unwrap() {
                    return Err(QIE::Overflow)
                }
                if v_2 < x.min_value().to_i128().unwrap() {
                    return Err(QIE::Underflow)
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
                    .ok_or(QIE::DivisionByZero)?
            } else {
                v_1
                    .checked_mul(scale)
                    .ok_or(QIE::Overflow)?
                    .checked_div(v_1)
                    .ok_or(QIE::DivisionByZero)?
            };
            if v_2 > x.max_value().to_u128().unwrap() {
                return Err(QIE::Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
            Ok(v_2)
        }
    }   
}

mod partial_ord {
    boiler::extend!();

    impl<const A: u8, B: PrimIntI + TrBranded> PartialOrd for Q<A, B> where PrecisionCheck<A>: PrecisionI {
        fn gt(&self, other: &Self) -> bool {
            self.v > other.v
        }
    
        fn lt(&self, other: &Self) -> bool {
            self.v < other.v
        }
    
        fn ge(&self, other: &Self) -> bool {
            self.v >= other.v
        }
    
        fn le(&self, other: &Self) -> bool {
            self.v <= other.v
        }
    
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}

mod ord {
    boiler::extend!();

    impl<const A: u8, B: PrimInt + Branded> Ord for Q<A, B> where CheckPrecision<A>: IsPrecision {
        fn clamp(self, min: Self, max: Self) -> Self {
            if self > max {
                return max
            }
            if self < min {
                return min
            }
            self
        }
    
        fn max(self, other: Self) -> Self {
            let x: &Self = &self;
            let y: &Self = &other;
            let v_0: B = x.v;
            let v_1: B = y.v;
            let v_2: B = v_0.max(v_1);
            if v_2 == v_0 {
                return q(v_0)
            } 
            q(v_1)
        }
    
        fn min(self, other: Self) -> Self {
            let x: &Self = &self;
            let y: &Self = &other;
            let v_0: B = x.v;
            let v_1: B = y.v;
            let v_2: B = v_0.min(v_1);
            if v_2 == v_0 {
                return q(v_0)
            }
            q(v_1)    
        }
    
        fn cmp(&self, other: &Self) -> Ordering {
            self.v.cmp(&other.v)
        }
    }
}

mod partial_eq {
    boiler::extend!();

    impl<const A: u8, B: PrimInt> PartialEq for Q<A, B> where CheckPrecision<A>: IsPrecision {
        fn eq(&self, other: &Self) -> bool {
            self.v == other.v
        }
    }
}

mod eq {
    boiler::extend!();

    impl<const A: u8, B: PrimInt> Eq for Q<A, B> where CheckPrecision<A>: IsPrecision {}
}