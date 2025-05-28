use super::*;

pub trait MuldivEngine {
    #[inline]
    fn muldiv<T>(x: T, y: T, z: T) -> Result<T> where T: int::Int {
        use Error::*;

        if z == T::N0 {
            return Err(DivisionByZero);
        }
        match (T::BIT, T::IS_SIGNED) {
            (_, true) if T::BIT <= 64 => {
                let n: T = x.checked_mul(&y).ok_or(Overflow)?;
                let n: T = n / z;
                Ok(n)
            },
            (_, false) if T::BIT < 128 => {
                let (a, b) = wide_mul::get(x, y)?;
                if b >= z {
                    Err(Overflow)
                } else if b == T::N0 {
                    Ok(a / z)
                } else {
                    Ok(fold::get(a, b, z)? / z)
                }
            },
            (128, _) => {
                let n: T = x.checked_mul(&y).ok_or(Overflow)?;
                Ok(n / z)
            },
            _ => unsafe {
                ::core::hint::unreachable_unchecked();
            }
        }
    }
}

impl<T> MuldivEngine for T where T: Engine {}