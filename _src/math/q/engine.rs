::boiler::extend!();

pub trait Engine    
where
    Self: Sized,
    Self: Clone {
    
    fn sqrt<const A: u8, B>(&self, x: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Int,
        B: int::Introspection {
        let precision: u32 = A.into();
        let base: u128 = 10;
        let scale: u128 = base.pow(precision);
        let x: B = x._v;
        if x.is_signed() {
            let x: i128 = x.to_i128().ok_or(Error::UnsupportedConversion)?;
            if x < 0 {
                return Err(Error::NegSqrt)
            }
            let x: u128 = x.to_u128().ok_or(Error::UnsupportedConversion)?;
            let x: u128 = x * scale;
            let x: u128 = _sqrt(x);
            let engine: Self = self.clone();
            let result: B = B::from(x).ok_or(Error::UnsupportedConversion)?;
            let result: Q<A, B, Self> = new_with_custom_engine(result, engine);
            return Ok(result)
        }
        let x: u128 = x.to_u128().ok_or(Error::UnsupportedConversion)?;
        let x: u128 = x * scale;
        let x: u128 = _sqrt(x);
        let engine: Self = self.clone();
        let result: B = B::from(x).ok_or(Error::UnsupportedConversion)?;
        let result: Q<A, B, Self> = new_with_custom_engine(result, engine);
        Ok(result)
    }

    fn cast<const A: u8, const B: u8, C>(&self, x: &Q<A, C, Self>) -> Result<Q<B, C, Self>> 
    where
        C: int::Int,
        C: int::Introspection {
        let x: C = x._v;
        if A == B {
            let engine: Self = self.clone();
            let r: C = x;
            let r: Q<B, C, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let old_precision: u32 = A.into();
        let new_precision: u32 = B.into();
        if x.is_signed() {
            let base: i128 = 10;
            let old_scale: i128 = base.pow(old_precision);
            let new_scale: i128 = base.pow(new_precision);
            let engine: Self = self.clone();
            let r: i128 = x.to_i128().unwrap();
            let r: i128 = self.muldiv(&r, &new_scale, &old_scale)?;
            let r: C = C::from(r).unwrap();
            let r: Q<B, C, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let base: u128 = 10;
        let old_scale: u128 = base.pow(old_precision);
        let new_scale: u128 = base.pow(new_precision);
        let engine: Self = self.clone();
        let r: u128 = x.to_u128().unwrap();
        let r: u128 = self.muldiv(&r, &new_scale, &old_scale)?;
        let r: C = C::from(r).unwrap();
        let r: Q<B, C, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn mul<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Int,
        B: int::Introspection {
        let x: B = x._v;
        let y: B = y._v;
        let precision: u32 = A.into();
        if x.is_signed() {
            let base: i128 = 10;
            let scale: i128 = base.pow(precision);
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            let engine: Self = self.clone();
            let r: i128 = self.muldiv(&x, &y, &scale)?;
            let r: B = B::from(r).unwrap();
            let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let base: u128 = 10;
        let scale: u128 = base.pow(precision);
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let r: u128 = self.muldiv(&x, &y, &scale)?;
        let engine: Self = self.clone();
        let r: B = B::from(r).unwrap();
        let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn div<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Int,
        B: int::Introspection {
        let x: B = x._v;
        let y: B = y._v;
        let precision: u32 = A.into();
        let base: u128 = 10;
        let scale: u128 = base.pow(precision);
        if x.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            if scale.is_power_of_two() {
                let z: u32 = scale.trailing_zeros();
                let engine: Self = self.clone();
                let r: i128 = (x << z).checked_div(y).ok_or(Error::DivByZero)?;
                let r: B = B::from(r).unwrap();
                let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
                return r.into_ok()
            }
            let scale: i128 = scale.to_i128().unwrap();
            let engine: Self = self.clone();
            let r: i128 = self.muldiv(&x, &scale, &y)?;
            let r: B = B::from(r).unwrap();
            let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        if scale.is_power_of_two() {
            let z: u32 = scale.trailing_zeros();
            let engine: Self = self.clone();
            let r: u128 = (x << z).checked_div(y).ok_or(Error::DivByZero)?;
            let r: B = B::from(r).unwrap();
            let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
            return r.into_ok()
        }
        let engine: Self = self.clone();
        let r: u128 = self.muldiv(&x, &scale, &y)?;
        let r: B = B::from(r).unwrap();
        let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn rem<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where
        B: int::Int,
        B: int::Introspection,
        B: num::CheckedRem {
        let x: B = x._v;
        let y: B = y._v;
        if y == B::zero() {
            return Error::DivByZero.into_err()
        }
        let r: B = x.checked_rem(&y).ok_or(Error::RemByZero)?;
        let engine: Self = self.clone();
        let r: Q<A, B, Self> = new_with_custom_engine(r, engine);
        r.into_ok()
    }

    fn add<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>> 
    where 
        B: int::Int,
        B: int::Introspection {
        let x: &Q<A, B, Self> = x;
        let x: B = x._v;
        let y: &Q<A, B, Self> = y;
        let y: B = y._v;
        let z: B = x.checked_add(&y).ok_or(Error::Overflow)?;
        let engine: Self = self.clone();
        let z: Q<A, B, Self> = new_with_custom_engine(z, engine);
        z.into_ok()
    }

    fn sub<const A: u8, B>(&self, x: &Q<A, B, Self>, y: &Q<A, B, Self>) -> Result<Q<A, B, Self>>
    where
        B: int::Int,
        B: int::Introspection {
        let x: &Q<A, B, Self> = x;
        let x: B = x._v;
        let y: &Q<A, B, Self> = y;
        let y: B = y._v;
        let z: B = x.checked_sub(&y).ok_or(Error::Overflow)?;
        let engine: Self = self.clone();
        let z: Q<A, B, Self> = new_with_custom_engine(z, engine);
        z.into_ok()
    }

    fn muldiv<'a, T>(&self, x: &'a T, y: &'a T, z: &'a T) -> Result<T>
    where
        T: int::Int,
        T: int::Introspection {
        if z == &T::zero() {
            return Error::DivByZero.into_err()
        }
        if z.is_signed() {
            let x: i128 = x.to_i128().unwrap();
            let y: i128 = y.to_i128().unwrap();
            let z: i128 = z.to_i128().unwrap();
            let sign: i128 = ((x ^y ^ z) >> 127) & 1;
            let x: u128 = x.unsigned_abs();
            let y: u128 = y.unsigned_abs();
            let z: u128 = z.unsigned_abs();
            let (a, b) = _wide_mul(x, y);
            if b >= z {
                return Error::Overflow.into_err()
            }
            if b == 0 {
                let r: u128 = a / z;
                if sign == 1 {
                    let r: T = r.to_i128()
                        .ok_or(Error::UnsupportedConversion)?
                        .wrapping_neg()
                        .to_int()
                        .ok_or(Error::UnsupportedConversion)?;
                    return r.into_ok()
                }
                let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
                return r.into_ok()
            }
            let r: u128 = _fold_128_bit_product_mod(a, b, z) / z;
            if sign == 1 {
                let r: T = r.to_i128()
                    .ok_or(Error::UnsupportedConversion)?
                    .wrapping_neg()
                    .to_int()
                    .ok_or(Error::UnsupportedConversion)?;
                return r.into_ok()
            }
            let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
            return r.into_ok()
        }
        let x: u128 = x.to_u128().unwrap();
        let y: u128 = y.to_u128().unwrap();
        let z: u128 = z.to_u128().unwrap();
        let (a, b) = _wide_mul(x, y);
        if b >= z {
            return Error::Overflow.into_err()
        }
        if b == 0 {
            let r: u128 = a / z;
            let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
            return r.into_ok()
        }
        let r: u128 = _fold_128_bit_product_mod(a, b, z) / z;
        let r: T = r.to_int().ok_or(Error::UnsupportedConversion)?;
        r.into_ok()
    }
}
fn _sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0
    }
    let mut x_0 = n / 2;
    let mut x_1 = (x_0 + n / x_0) / 2;
    while x_1 < x_0 {
        x_0 = x_1;
        x_1 = (x_0 + n / x_0) / 2;
    }
    x_0
}

fn _fold_128_bit_product_mod(a: u128, b: u128, z: u128) -> u128 {
    (((((b % z) << 64) | (a >> 64)) % z) << 64) | (a & 0xFFFFFFFFFFFFFFFF)
}

fn _wide_mul(x: u128, y: u128) -> (u128, u128) {
    let x_hi: u128 = x >> 64;
    let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
    let y_hi: u128 = y >> 64;
    let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
    let lo_lo: u128 = x_lo * y_lo;
    let lo_hi: u128 = x_lo * y_hi;
    let hi_lo: u128 = x_hi * y_lo;
    let hi_hi: u128 = x_hi * y_hi;
    let m: u128 = lo_hi + hi_lo;
    let c: u128 = ((m < lo_hi) as u128) << 64;
    let m_lo: u128 = m << 64;
    let m_hi: u128 = m >> 64;
    let a: u128 = lo_lo.wrapping_add(m_lo);
    let b: u128 = hi_hi + m_hi + c + ((a < lo_lo) as u128);
    (a, b)
}