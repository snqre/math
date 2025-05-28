use super::*;

#[inline]
pub fn get<T>(x: T, y: T, z: T) -> Result<T> where T: int::Int {
    if T::IS_SIGNED {
        return a(x, y, z)
    }
    b(x, y, z)
}

#[inline]
fn a<T>(x: T, y: T, z: T) -> Result<T> where T: int::Int {
    use Error::*;
    
    let (x, y, z) = unsafe {
        let x: i128 = x.to_i128().unwrap_unchecked();
        let y: i128 = y.to_i128().unwrap_unchecked();
        let z: i128 = z.to_i128().unwrap_unchecked();
        (x, y, z)
    };
    let n: i128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if n > T::MAX_I128 {
        return Err(Overflow)
    }
    if n < T::MIN_I128 {
        return Err(Underflow)
    }
    let n: T = unsafe {
        T::from(n).unwrap_unchecked()
    };
    Ok(n)
}

#[inline]
fn b<T>(x: T, y: T, z: T) -> Result<T> where T: int::Int {
    use Error::*;
    
    let (x, y, z) = unsafe {
        let x: u128 = x.to_u128().unwrap_unchecked();
        let y: u128 = y.to_u128().unwrap_unchecked();
        let z: u128 = z.to_u128().unwrap_unchecked();
        (x, y, z)
    };
    let n: u128 = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
    if n > T::MAX_U128 {
        return Err(Overflow)
    }
    if n < T::MIN_U128 {
        return Err(Underflow)
    }
    let n: T = unsafe {
        T::from(n).unwrap_unchecked()
    };
    Ok(n)
}