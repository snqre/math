use super::*;

#[inline]
pub fn muldiv<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: int::Int {
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
            hint::unreachable_unchecked();
        }
    }
}

#[inline]
fn fold<T>(x: T, y: T, z: T) -> Result<T> where T: int::Int {
    if T::IS_SIGNED {
        return fold_signed(x, y, z);
    }
    fold_unsigned(x, y, z)
}

#[inline]
fn fold_signed<T>(x: T, y: T, z: T) -> Result<T> where T: int::Int {
    let (x, y, z) = unsafe {
        let x: i128 = x.to_i128().unwrap_unchecked();
        let y: i128 = y.to_i128().unwrap_unchecked();
        let z: i128 = z.to_i128().unwrap_unchecked();
        (x, y, z)
    };
    let ret: i128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if ret > T::MAX_I128 {
        return Error::Overflow.into_err();
    }
    if ret < T::MIN_I128 {
        return Error::Underflow.into_err();
    }
    let ret: T = unsafe {
        T::from(ret).unwrap_unchecked()
    };
    ret.into_ok()
}

#[inline]
fn fold_unsigned<T>(x: T, y: T, z: T) -> Result<T> where T: int::Int {
    let (x, y, z) = unsafe {
        let x: u128 = x.to_u128().unwrap_unchecked();
        let y: u128 = y.to_u128().unwrap_unchecked();
        let z: u128 = z.to_u128().unwrap_unchecked();
        (x, y, z)
    };
    let r: u128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if r > T::MAX_U128 {
        return Error::Overflow.into_err()
    }
    if r < T::MIN_U128 {
        return Error::Underflow.into_err()
    }
    let r: T = unsafe {
        T::from(r).unwrap_unchecked()
    };
    r.into_ok()
}

#[inline]
fn wide_mul<T>(x: T, y: T) -> Result<(T, T)> where T: int::Int {
    if T::IS_SIGNED {
        return wide_mul_signed(x, y)
    }
    wide_mul_unsigned(x, y)
}

#[inline]
fn wide_mul_signed<T>(x: T, y: T) -> Result<(T, T)> where T: int::Int {
    assert!(T::IS_SIGNED);
    assert!(T::BITS_USIZE <= 64);
    let n: usize = T::BITS_USIZE / T::TWO_USIZE;
    let mask: T = (T::ONE << n) - T::ONE;
    let (lo_lo, lo_hi, hi_lo, hi_hi) = {
        let x_lo: T = x & mask;
        let x_hi: T = x >> n;
        let y_lo: T = y & mask;
        let y_hi: T = y >> n;
        let lo_lo: T = x_lo.checked_mul(&y_lo).ok_or(Error::Overflow)?;
        let lo_hi: T = x_lo.checked_mul(&y_hi).ok_or(Error::Overflow)?;
        let hi_lo: T = x_hi.checked_mul(&y_lo).ok_or(Error::Overflow)?;
        let hi_hi: T = x_hi.checked_mul(&y_hi).ok_or(Error::Overflow)?;
        (lo_lo, lo_hi, hi_lo, hi_hi)
    };
    let a: T = lo_hi.checked_add(&hi_lo).ok_or(Error::Overflow)?;
    let b: T = a << n;
    let hi: T = if lo_lo > lo_lo.wrapping_add(&b) {
        T::ONE
    } else {
        T::ZERO
    };
    let hi: T = hi_hi
        .checked_add(&(a >> n))
        .ok_or(Error::Overflow)?
        .checked_add(&hi)
        .ok_or(Error::Overflow)?;
    let lo: T = lo_lo.wrapping_add(&b);
    (lo, hi).into_ok()
}

#[inline]
fn wide_mul_unsigned<T>(x: T, y: T) -> Result<(T, T)> where T: int::Int, {
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
            return (a, b).into_ok()
        }
    }
    if a > T::MAX_U128 {
        return Error::Overflow.into_err()
    }
    if a < T::MIN_U128 {
        return Error::Underflow.into_err()
    }
    if b > T::MAX_U128 {
        return Error::Overflow.into_err()
    }
    if b < T::MIN_U128 {
        return Error::Underflow.into_err()
    }
    let (a, b) = unsafe {
        let a: T = T::from(a).unwrap_unchecked();
        let b: T = T::from(b).unwrap_unchecked();
        (a, b)
    };
    (a, b).into_ok()
}