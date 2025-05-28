use super::*;

#[inline]
pub fn get<T>(x: T, y: T) -> Result<(T, T)> where T: int::Int {
    if T::IS_SIGNED {
        return a(x, y)
    } 
    b(x, y)
}

#[inline]
fn a<T>(x: T, y: T) -> Result<(T, T)> where T: int::Int {
    use Error::*;
    
    assert!(T::IS_SIGNED);
    assert!(T::BIT <= 64);
    let a: usize = T::BIT as usize;
    let b: u8 = T::N2_U128.try_into().unwrap();
    let b: usize = b as usize;
    let n: usize = a / b;
    let mask: T = (T::N1 << n) - T::N1;
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
        T::N1
    } else {
        T::N0
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
fn b<T>(x: T, y: T) -> Result<(T, T)> where T: int::Int, {
    use Error::*;
    
    assert!(!T::IS_SIGNED);
    assert!(T::BIT <= 64);
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
    if T::BIT == 128 {
        unsafe {
            let a: T = T::from(a).unwrap_unchecked();
            let b: T = T::from(b).unwrap_unchecked();
            return Ok((a, b))
        }
    }
    if a > T::MAX_U128 {
        return Err(Overflow)
    }
    if a < T::MIN_U128 {
        return Err(Underflow)
    }
    if b > T::MAX_U128 {
        return Err(Overflow)
    }
    if b < T::MIN_U128 {
        return Err(Underflow)
    }
    let (a, b) = unsafe {
        let a: T = T::from(a).unwrap_unchecked();
        let b: T = T::from(b).unwrap_unchecked();
        (a, b)
    };
    Ok((a, b))
}