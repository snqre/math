use super::*;

#[inline]
pub fn calculate<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: int::Int {
    if T::IS_SIGNED {
        return a(x, y, z);
    }
    b(x, y, z)
}

#[inline]
fn a<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: int::Int {
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
fn b<T>(x: T, y: T, z: T) -> Result<T> 
where 
    T: int::Int {
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