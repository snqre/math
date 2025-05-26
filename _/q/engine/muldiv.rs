use super::*;

pub trait Handler {
    #[inline]
    fn muldiv<T>(&self, x: T, y: T, z: T) -> Result<T> 
    where
        T: num::Int {
        if z == T::ZERO {
            return Err(Error::DivisionByZero);
        }
        match (T::BITS_USIZE, T::IS_SIGNED) {
            (_, true) if T::BITS_USIZE <= 64 => {
                let ret: T = x.checked_mul(&y).ok_or(Error::Overflow)?;
                let ret: T = ret / z;
                Ok(ret)
            },
            (_, false) if T::BITS_USIZE < 128 => {
                let (a, b) = wide_mul(x, y)?;
                if b >= z {
                    return Err(Error::Overflow);
                }
                if b == T::ZERO {
                    return Ok(a / z);
                }
                Ok(fold(a, b, z)? / z)
            },
            (128, _) => {
                let ret: T = x.checked_mul(&y).ok_or(Error::Overflow)?;
                Ok(ret / z)
            },
            _ => unsafe {
                ::core::hint::unreachable_unchecked();
            }
        }
    }
}

#[inline]
fn fold<T>(x: T, y: T, z: T) -> Result<T> 
where
    T: num::Int {
    if T::IS_SIGNED {
        return fold_signed(x, y, z);
    }
    fold_unsigned(x, y, z)
}

#[inline]
fn fold_signed<T>(x: T, y: T, z: T) -> Result<T> 
where
    T: num::Int {
    let (x, y, z) = unsafe {
        let x: i128 = x.to_i128().unwrap_unchecked();
        let y: i128 = y.to_i128().unwrap_unchecked();
        let z: i128 = z.to_i128().unwrap_unchecked();
        (x, y, z)
    };
    let out: i128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if out > T::MAX_I128 {
        return Err(Error::Overflow);
    }
    if out < T::MIN_I128 {
        return Err(Error::Underflow);
    }
    let out: T = unsafe {
        T::from(out).unwrap_unchecked()
    };
    Ok(out)
}

#[inline]
fn fold_unsigned<T>(x: T, y: T, z: T) -> Result<T> 
where
    T: num::Int {
    let (x, y, z) = unsafe {
        let x: u128 = x.to_u128().unwrap_unchecked();
        let y: u128 = y.to_u128().unwrap_unchecked();
        let z: u128 = z.to_u128().unwrap_unchecked();
        (x, y, z)
    };
    let out: u128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if out > T::MAX_U128 {
        return Err(Error::Overflow);
    }
    if out < T::MIN_U128 {
        return Err(Error::Underflow);
    }
    let out: T = unsafe {
        T::from(out).unwrap_unchecked()
    };
    Ok(out)
}

#[inline]
fn wide_mul<T: int::Int>(x: T, y: T) -> Result<(T, T)> {
    if T::IS_SIGNED {
        return wide_mul_signed(x, y)
    }
    wide_mul_unsigned(x, y)
}

#[inline]
fn wide_mul_signed<T: int::Int>(x: T, y: T) -> Result<(T, T)> {
    assert!(T::IS_SIGNED);
    assert!(T::BITS_USIZE <= 64);
    let n: usize = T::BITS_USIZE / T::TWO_USIZE;
    let mask: T = (T::ONE << n) - T::ONE;
    let (lo_lo, lo_hi, hi_lo, hi_hi) = {
        let x_lo: T = x & mask;
        let x_hi: T = x >> n;
        let y_lo: T = y & mask;
        let y_hi: T = y >> n;
        let lo_lo: T = x_lo.checked_mul(&y_lo).ok_or(Overflow)?;
        let lo_hi: T = x_lo.checked_mul(&y_hi).ok_or(Overflow)?;
        let hi_lo: T = x_hi.checked_mul(&y_lo).ok_or(Overflow)?;
        let hi_hi: T = x_hi.checked_mul(&y_hi).ok_or(Overflow)?;
        (lo_lo, lo_hi, hi_lo, hi_hi)
    };
    let a: T = lo_hi.checked_add(&hi_lo).ok_or(Overflow)?;
    let b: T = a << n;
    let hi: T = if lo_lo > lo_lo.wrapping_add(&b) {
        T::ONE
    } else {
        T::ZERO
    };
    let hi: T = hi_hi
        .checked_add(&(a >> n))
        .ok_or(Overflow)?
        .checked_add(&hi)
        .ok_or(Overflow)?;
    let lo: T = lo_lo.wrapping_add(&b);
    Ok((lo, hi))
}

#[inline]
fn wide_mul_unsigned<T: int::Int>(x: T, y: T) -> Result<(T, T)> {
    assert!(T::IS_UNSIGNED);
    assert!(T::BITS_USIZE <= 64);
    let (x, y) = unsafe {
        let x: u128 = x.to_u128().unwrap_unchecked();
        let y: u128 = y.to_u128().unwrap_unchecked();
        (x, y)
    };
    let (a, b) = {
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
    };
    if T::BITS_USIZE == 128 {
        unsafe {
            let a: T = T::from(a).unwrap_unchecked();
            let b: T = T::from(b).unwrap_unchecked();
            return Ok((a, b));
        }
    }
    if a > T::MAX_U128 {
        return Err(Overflow);
    }
    if a < T::MIN_U128 {
        return Err(Underflow);
    }
    if b > T::MAX_U128 {
        return Err(Overflow);
    }
    if b < T::MIN_U128 {
        return Err(Underflow);
    }
    let (a, b) = unsafe {
        let a: T = T::from(a).unwrap_unchecked();
        let b: T = T::from(b).unwrap_unchecked();
        (a, b)
    };
    Ok((a, b))
}

impl<T> Handler for T 
where
    T: super::Handler 
{}