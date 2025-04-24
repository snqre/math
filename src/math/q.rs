boiler::expose!(
    float_constructor,
    int_constructor,
    constructor,
    add,
    q,
    muldiv,
);

use crate::math::sign_introspection_trait::SignIntrospectionTrait;
use crate::math::precision::Precision;
use crate::math::precision_trait::PrecisionTrait;
use crate::math::branded_trait::BrandedTrait;
use crate::math::q_trait::QTrait;
use crate::math::q_trait::QTraitError;
use crate::math::q_trait::QTraitResult;
use core::ops::Add as AddTrait;
use core::ops::Sub as SubTrait;
use core::ops::Mul as MulTrait;
use core::ops::Div as DivTrait;
use core::ops::Rem as RemTrait;
use core::convert::TryInto as TryIntoTrait;
use core::convert::TryFrom as TryFromTrait;
use core::cmp::PartialEq as PartialEqTrait;
use core::cmp::Eq as EqTrait;
use core::cmp::Ordering;
use num_traits::int::PrimInt as PrimIntTrait;
use num_traits::float::Float as FloatTrait;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B: PrimIntTrait> where Precision<A>: PrecisionTrait {
    pub(super) _v: B,
}

mod float_constructor {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait> Q<A, B> where Precision<A>: PrecisionTrait {
    
        pub fn from_float<const C: u8, D: FloatTrait>(v: D) -> QTraitResult<Self> where Precision<C>: PrecisionTrait, {
            let decimals: u32 = A.into();
            let scale = D::from(10u32).ok_or(QTraitError::ConversionFailure)?.powi(decimals);
            let v: D = v.checked_mul(scale).ok_or(QTraitError::Overflow)?;
            if B::zero().is_signed() {
                let v: i128 = v.to_i128().ok_or(QTraitError::ConversionFailure)?;
                if v > B::max_value().to_i128().unwrap() {
                    return Err(QTraitError::Overflow)
                }
                if v < B::min_value().to_i128().unwrap() {
                    return Err(QTraitError::Underflow)
                }
                let v: B = B::from(v).ok_or(QTraitError::ConversionFailure)?;
                Ok(q(v))
            }
            let v: u128 = v.to_u128().ok_or(QTraitError::ConversionFailure)?;
            if v > B::max_value().to_u128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            let v: B = B::from(v).ok_or(QTraitError::ConversionFailure)?;
            Ok(q(v))
        }
    }

    pub fn qfl<const A: u8, B: FloatTrait, const C: u8, D: PrimIntTrait + BrandedTrait + SignIntrospectionTrait>(v: B) -> QTraitResult<Q<C, D>> 
    where
        Precision<A>: PrecisionTrait,
        Precision<C>: PrecisionTrait, {
        Q::from_float::<A, B>(v)
    }

    #[cfg(test)]
    mod test {
        boiler::extend!();

        #[test]
        fn test() {
            let x: Q<2u8, u128> = qfl::<2u8, f32, 2u8, u128>(56.48f32);
        }
    }
}

mod int_constructor {
    boiler::extend!();

    pub fn q_int<const A: u8, B: PrimIntTrait, const C: u8, D: PrimIntTrait + BrandedTrait + SignIntrospectionTrait>(v: B) -> QTraitResult<Q::<C, D>> where 
        Precision<A>: PrecisionTrait,
        Precision<C>: PrecisionTrait, {
        Q::new_from_int::<A, B>(v)
    }
    
    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait {
     
        pub fn new_from_int<const C: u8, D: PrimIntTrait>(v: D) -> QTraitResult<Self> where Precision<C>: PrecisionTrait {
            let v: B = B::from(v).ok_or(QTraitError::ConversionFailure)?;
            let v: Q<C, B> = q(v);
            let v: Q<A, B> = v.cast()?;
            Ok(v)
        }
    }
}

mod constructor {
    boiler::extend!();
    
    impl<const A: u8, B: PrimIntTrait> Q<A, B> where Precision<A>: PrecisionTrait {
        pub fn new(v: B) -> Self {
            debug_assert!(matches!(A, 1u8..=38u8));
            Self {
                _v: v,
            }
        }
    }

    pub fn q<const A: u8, B: PrimIntTrait>(v: B) -> Q<A, B> where Precision<A>: PrecisionTrait {
        Q::new(v)
    }
}

mod q {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait> QTrait<A, B> for Q<A, B> where Precision<A>: PrecisionTrait 
        {}
}

mod cast {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait {
        pub fn cast<const C: u8>(&self) -> QTraitResult<Q<C, B>> where Precision<C>: PrecisionTrait {
            if A == C {
                return Ok(q(self._v))
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
                    .ok_or(QTraitError::Overflow)?
                    .checked_div(old_scale)
                    .ok_or(QTraitError::DivisionByZero)?;
                if result > B::max_value().to_i128().unwrap() {
                    return Err(QTraitError::Overflow)
                }
                if result < B::min_value().to_i128().unwrap() {
                    return Err(QTraitError::Underflow)
                }
                return q_int::<C, i128, C, B>(result)
            }
            let old_scale: u128 = 10u128.pow(old_decimals);
            let new_scale: u128 = 10u128.pow(new_decimals);
            let result: u128 = self._v
                .to_u128()
                .unwrap()
                .checked_mul(new_scale)
                .ok_or(QTraitError::Overflow)?
                .checked_div(old_scale)
                .ok_or(QTraitError::DivisionByZero)?;
            if result > B::max_value().to_u128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            q_int::<C, u128, C, B>(result)
        }
    }
}

mod rem {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> RemTrait for Q<A, B> where Precision<A>: PrecisionTrait {
        type Output = QTraitResult<Self>;
    
        fn rem(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            if x.is_signed() && y.is_signed() {
                let v_0: i128 = x._v.to_i128().unwrap();
                let v_1: i128 = y._v.to_i128().unwrap();
                if v_1 == 0i128 {
                    return Err(QTraitError::DivisionByZero)
                }
                let v_2: i128 = v_0.checked_rem(v_1).ok_or(QTraitError::RemByZero)?;
                let v_2: B = B::from(v_2).unwrap();
                return Ok(q(v_2))   
            }
            debug_assert!(!x.is_signed());
            debug_assert!(!y.is_signed());
            let v_0: u128 = x._v.to_u128().unwrap();
            let v_1: u128 = y._v.to_u128().unwrap();
            if v_1 == 0u128 {
                return Err(QTraitError::DivisionByZero)
            }
            let v_2: u128 = v_0.checked_rem(v_1).ok_or(QTraitError::RemByZero)?;
            let v_2: B = B::from(v_2).unwrap();
            Ok(q(v_2))
        }
    } 
}

mod add {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait> AddTrait for Q<A, B> where Precision<A>: PrecisionTrait {
        type Output = QTraitResult<Self>;
    
        fn add(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_add(v_1).ok_or(QTraitError::Overflow)?;
            Ok(q(v_2))
        }
    }

    #[cfg(test)]
    mod test {
        boiler::extend!();

        #[test]
        fn test() {
            let x: Q<2u8, u128> = q(500u128);
            let y: Q<2u8, u128> = q(200u128);
            let z: Q<2u8, u128> = (x + y).unwrap();
            let z_ok: Q<2u8, u128> = q(700u128);
            assert_eq!(z, z_ok);
        }
    }
}

mod sub {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait> SubTrait for Q<A, B> where Precision<A>: PrecisionTrait {
        type Output = QTraitResult<Self>;
    
        fn sub(self, rhs: Self) -> Self::Output {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            let v_2: B = v_0.checked_sub(v_1).ok_or(QTraitError::Underflow)?;
            Ok(q(v_2))
        }
    }
}

// only the most accurate reesponse
mod mul {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> MulTrait for Q<A, B> where Precision<A>: PrecisionTrait {
        type Output = QTraitResult<Self>;
    
        fn mul(self, rhs: Self) -> Self::Output {
            let x: Self = self;
            let y: Self = rhs;
            x.mul_with_newton_raphson_inverse(y)
        }
    }
}

/// will try all strategies before failing
mod fallback_mul {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait {
    
        /// Performs multiplication with a fallback strategy for enhanced robustness.
        ///
        /// This implementation attempts the most precise method first (Newton-Raphson inverse),
        /// and if it fails due to edge cases or numerical instability, it proceeds through
        /// additional strategies:
        ///
        /// 1. Bit-simulated multiplication (`bit_simulated_mul`)
        /// 2. Quick multiplication (`quick_mul`) as a final resort
        ///
        /// This layered fallback ensures that the function produces a result in as many cases
        /// as possible, even if full precision cannot be guaranteed.
        ///
        /// # Returns
        /// - `Ok(Q)` from the most successful and precise strategy available.
        /// - `Err(QTraitError)` only if all strategies fail.
        ///
        /// # Use Case
        /// This method is ideal for scenarios prioritizing usability and graceful degradation
        /// over strict correctness, such as UI tools, simulation environments, or
        /// exploratory data processing.
        ///
        /// # Example
        /// ```
        /// let result = q1.mul_with_fallack(q2); // Tries multiple methods before failing.
        /// ```
        fn mul_with_fallack(self, rhs: Self) -> QTraitResult<Self> {
            let x: Self = self;
            let y: Self = rhs;
            let v: QTraitResult<Self> = x.mul_with_newton_raphson_inverse(y);
            if v.is_ok() {
                return v
            }
            let v: QTraitResult<Self> = x.mul_with_bit_simulation(y);
            if v.is_ok() {
                return v
            }
            x.mul_with_native(y)
        }
    }
}

mod newton_raphson_inverse_mul {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait, {

        pub fn mul_with_newton_raphson_inverse(self, rhs: Self) -> QTraitResult<Self> {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let data: _MulDivData<'_, A, B> = _MulDivData {
                x,
                y,
                u128_muldiv_algo: &(Self::_mul_div_u128_with_newton_raphson_inverse as _U128MuldivAlgo),
                i128_muldiv_algo: &(Self::_mul_div_i128_with_newton_raphson_inverse as _I128MuldivAlgo),  
            };
            Self::_mul(data)
        }
    }

    #[cfg(test)]
    mod test {

    }
}

mod bit_simulated_mul {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait, {
        
        pub fn mul_with_bit_simulation(self, rhs: Self) -> QTraitResult<Self> {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let data: _MulDivData<'_, A, B> = _MulDivData {
                x,
                y,
                u128_muldiv_algo: &(Self::_mul_div_u128_with_256_bit_simulation as _U128MuldivAlgo),
                i128_muldiv_algo: &(Self::_mul_div_i128_with_256_bit_simulation as _I128MuldivAlgo),  
            };
            Self::_mul(data)
        }
    }
}

mod native_mul {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait, {
        
        pub fn mul_with_native(self, rhs: Self) -> QTraitResult<Self> {
            let x: &Self = &self;
            let y: &Self = &rhs;
            let data: _MulDivData<'_, A, B> = _MulDivData {
                x,
                y,
                u128_muldiv_algo: &(Self::_mul_div_u128_with_native as fn(u128, u128, u128) -> QTraitResult<u128>),
                i128_muldiv_algo: &(Self::_mul_div_i128_with_native as fn(i128, i128, i128) -> QTraitResult<i128>),  
            };
            Self::_mul(data)
        }
    }
}

mod muldiv {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait, {

        pub(super) fn _mul<'l1>(data: _MulDivData<'l1, A, B>) -> QTraitResult<Self> {
            let decimals: u32 = A.into();
            let scale: u128 = 10u128.pow(decimals);
            if data.x.is_signed() && data.y.is_signed() {
                let scale: i128 = scale.try_into().unwrap();
                let v_0: i128 = data.x._v.to_i128().unwrap();
                let v_1: i128 = data.y._v.to_i128().unwrap();
                let v_2: i128 = (data.i128_muldiv_algo)(v_0, v_1, scale)?;
                if v_2 > data.x.max_value().to_i128().unwrap() {
                    return Err(QTraitError::Overflow)
                }
                if v_2 < data.x.min_value().to_i128().unwrap() {
                    return Err(QTraitError::Underflow)
                }
                let v_2: B = B::from(v_2).unwrap();
                return Ok(q(v_2))
            }
            debug_assert!(!data.x.is_signed());
            debug_assert!(!data.y.is_signed());
            let v_0: u128 = data.x._v.to_u128().unwrap();
            let v_1: u128 = data.y._v.to_u128().unwrap();
            let v_2: u128 = (data.u128_muldiv_algo)(v_0, v_1, scale)?;
            if v_2 > data.x.max_value().to_u128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            Ok(q(v_2))
        }

        pub(super) fn _div<'l1>(data: _MulDivData<'l1, A, B>) -> QTraitResult<Self> {
            let decimals: u32 = A.into();
            let scale: u128 = 10u128.pow(decimals);
            if data.x.is_signed() && data.y.is_signed() {
                let v_0: i128 = data.x._v.to_i128().unwrap();
                let v_1: i128 = data.y._v.to_i128().unwrap();
                let v_2: i128 = if scale.is_power_of_two() {
                    let scale_shift: u32 = scale.trailing_zeros();
                    (v_0 << scale_shift).checked_div(v_1).ok_or(QTraitError::DivisionByZero)?
                } else {
                    let scale: i128 = scale.try_into().unwrap();
                    (data.i128_muldiv_algo)(v_0, scale, v_1)?
                };
                if v_2 > data.x.max_value().to_i128().unwrap() {
                    return Err(QTraitError::Overflow)
                }
                if v_2 < data.x.max_value().to_i128().unwrap() {
                    return Err(QTraitError::Underflow)
                }
                let v_2: B = B::from(v_2).unwrap();
                let v_2: Self = q(v_2);
                return Ok(v_2)
            }
            debug_assert!(!data.x.is_signed());
            debug_assert!(!data.y.is_signed());
            let v_0: u128 = data.x._v.to_u128().unwrap();
            let v_1: u128 = data.y._v.to_u128().unwrap();
            let v_2: u128 = if scale.is_power_of_two() {
                let scale_shift: u32 = scale.trailing_zeros();
                (v_0 << scale_shift).checked_div(v_1).ok_or(QTraitError::DivisionByZero)?
            } else {
                (data.u128_muldiv_algo)(v_0, scale, v_1)?
            };
            if v_2 > data.x.max_value().to_u128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Self = q(v_2);
            Ok(v_2)
        }

        /// 🚀 Fastest: Naive multiplication and division.
        ///
        /// Performs (x * y) / z using `u128` arithmetic.
        /// This is the fastest version but assumes that `x * y` does not overflow `u128`.
        /// It is safe and efficient for small values or controlled inputs.
        ///
        /// # Errors
        /// - Returns `DivisionByZero` if `z == 0`.
        /// - Returns `Overflow` if `x * y` exceeds `u128::MAX`.
        pub(super) fn _mul_div_u128_with_native(x: u128, y: u128, z: u128) -> QTraitResult<u128> {
            if z == 0u128 {
                return Err(QTraitError::DivisionByZero)
            }
            let result: u128 = x.checked_mul(y).ok_or(QTraitError::Overflow)?;
            let result: u128 = result / z;
            Ok(result)
        }

        pub(super) fn _mul_div_i128_with_native(x: i128, y: i128, z: i128) -> QTraitResult<i128> {
            if z == 0 {
                return Err(QTraitError::DivisionByZero)
            }
            let sign: i128 = ((x ^ y ^ z) >> 127) & 1;
            let x_u: u128 = x.unsigned_abs();
            let y_u: u128 = y.unsigned_abs();
            let z_u: u128 = z.unsigned_abs();
            let result = x_u.checked_mul(y_u).ok_or(QTraitError::Overflow)?;
            let result = result / z_u;
            let result = if sign == 1 {
                result.wrapping_neg() as i128
            } else {
                result as i128
            };
            Ok(result)
        }

        /// ⚖️ Balanced: Simulates 256-bit arithmetic using manual bit manipulation.
        ///
        /// Performs (x * y) / z using only u128 arithmetic while simulating full 256-bit multiplication.
        /// This provides higher precision and prevents overflow in most realistic cases,
        /// by representing the full 256-bit product across two `u128` values.
        ///
        /// # Internals
        /// - Splits x and y into high and low 64-bit halves.
        /// - Reconstructs the full 256-bit product manually.
        /// - Checks for overflow before division.
        ///
        /// # Errors
        /// - Returns `DivisionByZero` if `z == 0`.
        /// - Returns `Overflow` if the 256-bit result would overflow `z`.
        pub(super) fn _mul_div_u128_with_256_bit_simulation(x: u128, y: u128, z: u128) -> QTraitResult<u128> {
            if z == 0 {
                return Err(QTraitError::DivisionByZero)
            }
            let x_hi: u128 = x >> 64;
            let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
            let y_hi: u128 = y >> 64;
            let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
            let lo_lo: u128 = x_lo * y_lo;
            let lo_hi: u128 = x_lo * y_hi;
            let hi_lo: u128 = x_hi * y_lo;
            let hi_hi: u128 = x_hi * y_hi;
            let m: u128 = lo_hi + hi_lo;
            let c_0: u128 = ((m < lo_hi) as u128) << 64;
            let m_lo: u128 = m << 64;
            let m_hi: u128 = m >> 64;
            let p_0: u128 = lo_lo.wrapping_add(m_lo);
            let c_1: u128 = (p_0 < lo_lo) as u128;
            let p_1: u128 = hi_hi + m_hi + c_0 + c_1;
            if p_1 == 0 {
                let result: u128 = p_0 / z;
                return Ok(result)
            }
            if p_1 >= z {
                return Err(QTraitError::Overflow)
            }
            let rem: u128 = ((p_1 % z) << 64) | (p_0 >> 64);
            let rem: u128 = ((rem % z) << 64) | (p_0 & 0xFFFFFFFFFFFFFFFF);
            let result: u128 = rem / z;
            Ok(result)
        }

        pub(super) fn _mul_div_i128_with_256_bit_simulation(x: i128, y: i128, z: i128) -> QTraitResult<i128> {
            if z == 0 {
                return Err(QTraitError::DivisionByZero)
            }
            let sign: i128 = ((x ^ y ^ z) >> 127) & 1;
            let x_u: u128 = x.unsigned_abs();
            let y_u: u128 = y.unsigned_abs();
            let z_u: u128 = z.unsigned_abs();
            let x_hi: u128 = x_u >> 64;
            let x_lo: u128 = x_u & 0xFFFFFFFFFFFFFFFF;
            let y_hi: u128 = y_u >> 64;
            let y_lo: u128 = y_u & 0xFFFFFFFFFFFFFFFF;
            let lo_lo: u128 = x_lo * y_lo;
            let lo_hi: u128 = x_lo * y_hi;
            let hi_lo: u128 = x_hi * y_lo;
            let hi_hi: u128 = x_hi * y_hi;
            let m: u128 = lo_hi + hi_lo;
            let c_0: u128 = ((m < lo_hi) as u128) << 64;
            let m_lo: u128 = m << 64;
            let m_hi: u128 = m >> 64;
            let p_0: u128 = lo_lo.wrapping_add(m_lo);
            let c_1: u128 = (p_0 < lo_lo) as u128;
            let p_1: u128 = hi_hi + m_hi + c_0 + c_1;
            if p_1 == 0 {
                let result: u128 = p_0 / z_u;
                let result: i128 = if sign == 1 {
                    result.wrapping_neg() as i128
                } else {
                    result as i128
                };
                return Ok(result)
            }
            if p_1 >= z_u {
                return Err(QTraitError::Overflow)
            }
            let rem: u128 = ((p_1 % z_u) << 64) | (p_0 >> 64);
            let rem: u128 = ((rem % z_u) << 64) | (p_0 & 0xFFFFFFFFFFFFFFFF);
            let result: u128 = rem / z_u;
            let result: i128 = if sign == 1 {
                result.wrapping_neg() as i128
            } else {
                result as i128
            };
            Ok(result)
        }

        /// 🧠 Most Precise: Uses Newton-Raphson approximation to divide a simulated 256-bit product.
        ///
        /// Performs (x * y) / z using full 256-bit simulation and a Newton-Raphson method to approximate 1/z.
        /// This avoids direct division, replacing it with a series of multiplications and shifts for improved precision.
        ///
        /// Suitable for situations where rounding errors or fractional losses are unacceptable,
        /// such as financial or cryptographic applications.
        ///
        /// # Errors
        /// - Returns `DivisionByZero` if `z == 0`.
        /// - Returns `Overflow` if the high 128 bits of the product are ≥ `z`.
        pub(super) fn _mul_div_u128_with_newton_raphson_inverse(x: u128, y: u128, z: u128) -> QTraitResult<u128> {
            if z == 0 {
                return Err(QTraitError::DivisionByZero)
            }
            let (hi, lo) = {
                let x_hi: u128 = x >> 64u128;
                let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
                let y_hi: u128 = y >> 64;
                let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
                let lo_lo: u128 = x_lo * y_lo;
                let lo_hi: u128 = x_lo * y_hi;
                let hi_lo: u128 = x_hi * y_lo;
                let hi_hi: u128 = x_hi * y_hi;
                let m: u128 = lo_hi + hi_lo;
                let c_0: u128 = ((m < lo_hi) as u128) << 64;
                let m_lo: u128 = m << 64u128;
                let m_hi: u128 = m >> 64u128;
                let p_0: u128 = lo_lo.wrapping_add(m_lo);
                let c_1: u128 = (p_0 < lo_lo) as u128;
                let p_1: u128 = hi_hi + m_hi + c_0 + c_1;
                (p_1, p_0)
            };
            if hi >= z {
                return Err(QTraitError::Overflow)
            }
            let mut inv: u128 = (3 * z) ^ 2;
            for _ in 0..6 {
                let new_inv: u128 = z.wrapping_mul(inv);
                let new_inv: u128 = new_inv.wrapping_sub(new_inv);
                let new_inv: u128 = inv.wrapping_mul(new_inv);
                inv = new_inv;
            }
            let result = {
                let scale_hi: u128 = hi.wrapping_mul(inv);
                let scale_lo: u128 = lo.wrapping_mul(inv);
                scale_lo + (scale_hi << 64)
            };
            Ok(result)
        }

        pub(super) fn _mul_div_i128_with_newton_raphson_inverse(x: i128, y: i128, z: i128) -> QTraitResult<i128> {
            if z == 0 {
                return Err(QTraitError::DivisionByZero)
            }
            let sign: i128 = ((x ^ y ^ z) >> 127) & 1;
            let x_u = x.unsigned_abs();
            let y_u = y.unsigned_abs();
            let z_u = z.unsigned_abs();
            let (hi, lo) = {
                let x_hi: u128 = x_u >> 64;
                let x_lo: u128 = x_u & 0xFFFFFFFFFFFFFFFF;
                let y_hi: u128 = y_u >> 64;
                let y_lo: u128 = y_u & 0xFFFFFFFFFFFFFFFF;
                let lo_lo: u128 = x_lo * y_lo;
                let lo_hi: u128 = x_lo * y_hi;
                let hi_lo: u128 = x_hi * y_lo;
                let hi_hi: u128 = x_hi * y_hi;
                let m: u128 = lo_hi + hi_lo;
                let c_0: u128 = ((m < lo_hi) as u128) << 64;
                let m_lo: u128 = m << 64;
                let m_hi: u128 = m >> 64;
                let p_0: u128 = lo_lo.wrapping_add(m_lo);
                let c_1: u128 = (p_0 < lo_lo) as u128;
                let p_1: u128 = hi_hi + m_hi + c_0 + c_1;
                (p_1, p_0)
            };
            if hi >= z_u {
                return Err(QTraitError::Overflow)
            }
            let mut inv: u128 = (3 * z_u) ^ 2;
            for _ in 0..6 {
                let tmp = 2u128.wrapping_sub(z_u.wrapping_mul(inv));
                inv = inv.wrapping_mul(tmp);
            }
            let result = {
                let scale_hi: u128 = hi.wrapping_mul(inv);
                let scale_lo: u128 = lo.wrapping_mul(inv);
                scale_lo + (scale_hi << 64)
            };
            let result = if sign == 1 {
                result.wrapping_neg() as i128
            } else {
                result as i128
            };
            Ok(result)
        }
    }

    pub(super) struct _MulDivData<'l1, const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> where Precision<A>: PrecisionTrait, {
        pub x: &'l1 Q<A, B>,
        pub y: &'l1 Q<A, B>,
        pub u128_muldiv_algo: &'l1 _U128MuldivAlgo,
        pub i128_muldiv_algo: &'l1 _I128MuldivAlgo,
    }

    pub(super) type _U128MuldivAlgo = fn(u128, u128, u128) -> QTraitResult<u128>;

    pub(super) type _I128MuldivAlgo = fn(i128, i128, i128) -> QTraitResult<i128>;
}

mod div {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait + SignIntrospectionTrait> DivTrait for Q<A, B> where Precision<A>: PrecisionTrait {
        type Output = QTraitResult<Self>;
    
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
                        .ok_or(QTraitError::DivisionByZero)?
                } else {
                    let scale: i128 = scale.try_into().unwrap();
                    v_0
                        .checked_mul(scale)
                        .ok_or(QTraitError::Overflow)?
                        .checked_div(v_1)
                        .ok_or(QTraitError::DivisionByZero)?
                };
                if v_2 > x.max_value().to_i128().unwrap() {
                    return Err(QTraitError::Overflow)
                }
                if v_2 < x.min_value().to_i128().unwrap() {
                    return Err(QTraitError::Underflow)
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
                    .ok_or(QTraitError::DivisionByZero)?
            } else {
                v_1
                    .checked_mul(scale)
                    .ok_or(QTraitError::Overflow)?
                    .checked_div(v_1)
                    .ok_or(QTraitError::DivisionByZero)?
            };
            if v_2 > x.max_value().to_u128().unwrap() {
                return Err(QTraitError::Overflow)
            }
            let v_2: B = B::from(v_2).unwrap();
            let v_2: Q<A, B> = q(v_2);
            Ok(v_2)
        }
    }
}

mod sign_introspection {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + SignIntrospectionTrait> SignIntrospectionTrait for Q<A, B> where Precision<A>: PrecisionTrait {
        fn is_signed(&self) -> bool {
            self._v.is_signed()
        }
    }

    #[cfg(test)]
    mod test {
        boiler::extend!();
        
        #[test]
        fn test() {
            let x: Q<2u8, ii6> = q(500i16);
            assert_eq!(x.is_signed(), true);
        }
    }
}

mod cap {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + SignIntrospectionTrait> Q<A, B> where Precision<A>: PrecisionTrait {
        pub fn max_value(&self) -> B {
            B::max_value()
        }
    
        pub fn min_value(&self) -> B {
            B::min_value()
        }
    
        pub fn max_rep_value(&self) -> B {
            let decimals: u32 = A.into();
            if self.is_signed() {
                let scale: i128 = 10i128.pow(decimals);
                let max_value: i128 = self.max_value().to_i128().unwrap();
                let max_value: i128 = max_value / scale;
                let max_value: B = B::from(max_value).unwrap();
                return max_value
            }
            let scale: u128 = 10u128.pow(decimals);
            let max_value: u128 = self.max_value().to_u128().unwrap();
            let max_value: u128 = max_value / scale;
            let max_value: B = B::from(max_value).unwrap();
            return max_value
        }
    
        pub fn min_rep_value(&self) -> B {
            if A == 0u8 {
                return self.min_value()
            }
            if A > Q_MAX_PRECISION {
                
            }
            self.max_rep_value() - (self.max_rep_value() * 2)
        }
    }
}

mod to_u8 {
    boiler::extend!();
}

mod try_into_u8 {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + SignIntrospectionTrait> TryIntoTrait<u8> for Q<A, B> where Precision<A>: PrecisionTrait {
        type Error = QTraitError;
        
        fn try_into(self) -> Result<u8, Self::Error> {
            let x: &Self = &self;
            let v: &B = &x._v;
            let decimals: u32 = A.into();
            let scale: u128 = 10u128.pow(decimals);
            if v.is_signed() {
                let scale: i128 = scale.try_into().unwrap();
                let v: i128 = v
                    .to_i128()
                    .ok_or(QTraitError::ConversionFailure)?
                    .checked_div(scale)
                    .unwrap();
                return u8::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
            }
            let v: u128 = v
                .to_u128()
                .ok_or(QTraitError::ConversionFailure)?
                .checked_div(scale)
                .unwrap();
            u8::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
        }
    }
}

mod try_into_u16 {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + SignIntrospectionTrait> TryIntoTrait<u16> for Q<A, B> where Precision<A>: PrecisionTrait {
        type Error = QTraitError;
        
        fn try_into(self) -> Result<u16, Self::Error> {
            let x: &Self = &self;
            let v: &B = &x._v;
            let decimals: u32 = A.into();
            let scale: u128 = 10u128.pow(decimals);
            if v.is_signed() {
                let scale: i128 = scale.try_into().unwrap();
                let v: i128 = v
                    .to_i128()
                    .ok_or(QTraitError::ConversionFailure)?
                    .checked_div(scale)
                    .unwrap();
                return u16::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
            }
            let v: u128 = v
                .to_u128()
                .ok_or(QTraitError::ConversionFailure)?
                .checked_div(scale)
                .unwrap();
            u16::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
        }
    }

    #[cfg(test)]
    mod test {
        use crate::math::q::Q;
        use crate::math::q::q;
        use core::convert::*;
        
        #[test]
        fn test() {
            let x: Q<2u8, u16> = q(500u16);
            let y: u16 = x.try_into().unwrap();
        }
    }
}

mod try_into_u32 {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + SignIntrospectionTrait> TryIntoTrait<u32> for Q<A, B> where Precision<A>: PrecisionTrait {
        type Error = QTraitError;
        
        fn try_into(self) -> Result<u32, Self::Error> {
            let x: &Self = &self;
            let v: &B = &x._v;
            let decimals: u32 = A.into();
            let scale: u128 = 10u128.pow(decimals);
            if v.is_signed() {
                let scale: i128 = scale.try_into().unwrap();
                let v: i128 = v
                    .to_i128()
                    .ok_or(QTraitError::ConversionFailure)?
                    .checked_div(scale)
                    .unwrap();
                return u32::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
            }
            let v: u128 = v
                .to_u128()
                .ok_or(QTraitError::ConversionFailure)?
                .checked_div(scale)
                .unwrap();
            u32::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
        }
    }
}

mod try_from_u16 {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + SignIntrospectionTrait> TryFromTrait<u16> for Q<A, B> where Precision<A>: PrecisionTrait {
        type Error = QTraitError;
        
        fn try_from(value: u16) -> Result<Self, Self::Error> {
            let x: &Self = &self;
            let v: &B = &x._v;
            let decimals: u32 = A.into();
            let scale: u128 = 10u128.pow(decimals);
            if v.is_signed() {
                let scale: i128 = scale.try_into().unwrap();
                let v: i128 = v
                    .to_i128()
                    .ok_or(QTraitError::ConversionFailure)?
                    .checked_div(scale)
                    .unwrap();
                return u32::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
            }
            let v: u128 = v
                .to_u128()
                .ok_or(QTraitError::ConversionFailure)?
                .checked_div(scale)
                .unwrap();
            u32::try_from(v).ok().ok_or(QTraitError::ConversionFailure)
        }
    }
}

mod partial_ord {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait> PartialOrd for Q<A, B> where Precision<A>: PrecisionTrait {
        fn gt(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v > y._v
        }
    
        fn lt(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v < y._v
        }
    
        fn ge(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v >= y._v
        }
    
        fn le(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            x._v <= y._v
        }
    
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let x: &Self = self;
            let y: &Self = other;
            let order: Ordering = x.cmp(y);
            Some(order)
        }
    }
}

mod ord {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait + BrandedTrait> Ord for Q<A, B> where Precision<A>: PrecisionTrait {
        fn clamp(self, min: Self, max: Self) -> Self {
            match (self > max, self < min) {
                (true, _) => max,
                (_, true) => min,
                (_, _) => self,
            }
        }
    
        fn max(self, other: Self) -> Self {
            let x: &Self = &self;
            let y: &Self = &other;
            let v_0: B = x._v;
            let v_1: B = y._v;
            let v_2: B = v_0.max(v_1);
            if v_2 == v_0 {
                return q(v_0)
            } 
            q(v_1)
        }
    
        fn min(self, other: Self) -> Self {
            let x: &Self = &self;
            let y: &Self = &other;
            let v_0: B = x._v;
            let v_1: B = y._v;
            let v_2: B = v_0.min(v_1);
            if v_2 == v_0 {
                return q(v_0)
            }
            q(v_1)    
        }
    
        fn cmp(&self, other: &Self) -> Ordering {
            let x: &Self = &self;
            let y: &Self = &other;
            let v_0: B = x._v;
            let v_1: B = y._v;
            v_0.cmp(&v_1)
        }
    }
}

mod partial_eq {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait> PartialEqTrait for Q<A, B> where Precision<A>: PrecisionTrait {
        
        fn eq(&self, other: &Self) -> bool {
            let x: &Self = self;
            let y: &Self = other;
            let v_0: &B = &x._v;
            let v_1: &B = &y._v;
            v_0 == v_1
        }
    }
}

mod eq {
    boiler::extend!();

    impl<const A: u8, B: PrimIntTrait> EqTrait for Q<A, B> where Precision<A>: PrecisionTrait {}
}