use num_traits::ToPrimitive;

use crate::{int::{self}, precision};








pub fn tan<const A: u8, B>(degree: B) -> Option<B>
where
    B: int::Int,
    precision::Precision<A>: precision::Compatible, {
    let s: B = sin::<A, B>(degree)?;
    let c: B = cos::<A, B>(degree)?;

 

    div::<A, B>(s, c)
}

pub fn sin<const A: u8, B>(degree: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    if B::IS_SIGNED {
        let scale: i128 = scale_i128::<A>();
        let degree_90: i128 = 90;
        let degree_90: i128 = degree_90.checked_mul(scale)?;
        let degree_90: B = B::from(degree_90)?;
        return cos(sub(degree_90, degree)?);
    }
    let scale: u128 = scale_u128::<A>();
    let degree_90: u128 = 90;
    let degree_90: u128 = degree_90.checked_mul(scale)?;
    let degree_90: B = B::from(degree_90)?;
    cos(sub(degree_90, degree)?)
}

pub fn cos<const A: u8, B>(degree: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    macro_rules! op {
        ($size:ty) => {
            paste::paste! { {
                let degree: $size = degree.[<to_ $size>]().unwrap();
                let radian: $size = to_radian(degree)?; // Convert degree to radian
                let scale: $size = [<scale_ $size>]::<A>(); // Get the scale for precision
                let pi: $size = [<pi_ $size>]::<A>(); // Get the value of pi for precision
                let pi_two: $size = pi.checked_mul(2)?; // 2 * pi
                let pi_half: $size = pi.checked_mul(2)? / 2; // pi/2

                // Normalize the angle to the range [-pi, pi]
                let mut x: $size = radian.checked_add(pi)? % pi_two;
                if x < 0 {
                    x = x.checked_add(pi_two)?; // Ensure it's positive
                }
                // Shift angle within [-pi, pi]
                if x > pi_half {
                    x = pi.checked_sub(x)?; // Reflect to positive side
                }
                if x < -pi_half {
                    x = (-pi).checked_sub(x)?; // Reflect to negative side
                }

                let mut term: $size = scale;
                let mut sum: $size = term;
                let mut sign: bool = true;
                let mut k: u32 = 1;
                loop {
                    term = muldiv(term, x, scale)?; // Multiply by x (x^2)
                    term = muldiv(term, x, scale)?; // Multiply again (x^4, x^6, ...)
                    term = term.checked_div((2 * k - 1) as $size * (2 * k) as $size)?; // Divide by factorial (odd terms)
                    
                    if term == 0 {
                        break;
                    }

                    if sign {
                        sum = sum.checked_add(term)?;
                    } else {
                        sum = sum.checked_sub(term)?;
                    }

                    sign = !sign;
                    k = k.checked_add(1).unwrap();
                }
                let result: B = B::from(sum)?;
                Some(result)
            } }
        };
    }
    if B::IS_SIGNED {
        op!(i128)
    } else {
        op!(i128)
    }
}

pub fn to_radian<const A: u8, B>(degree: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    if B::IS_SIGNED {
        let degree: i128 = degree.to_i128().unwrap();
        let scale: i128 = scale_i128::<A>();
        let pi: i128 = pi_i128::<A>();
        let n: i128 = 180;
        let n: i128 = n.checked_mul(scale)?;
        let result: i128 = muldiv(degree, pi, n)?;
        let result: B = B::from(result)?;
        return Some(result);
    }
    let degree: u128 = degree.to_u128().unwrap();
    let scale: u128 = scale_u128::<A>();
    let pi: u128 = pi_u128::<A>();
    let n: u128 = 180;
    let n: u128 = n.checked_mul(scale)?;
    let result: u128 = muldiv(degree, pi, n)?;
    let result: B = B::from(result)?;
    Some(result)
}

pub fn to_degree<const A: u8, B>(radian: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    if B::IS_SIGNED {
        let radian: i128 = radian.to_i128().unwrap();
        let scale: i128 = scale_i128::<A>();
        let pi: i128 = pi_i128::<A>();
        let n: i128 = 180;
        let n: i128 = n.checked_mul(scale)?;
        let degree: i128 = muldiv(radian, n, pi)?;
        let degree: B = B::from(degree)?;
        return Some(degree);
    }
    let radian: u128 = radian.to_u128().unwrap();
    let scale: u128 = scale_u128::<A>();
    let pi: u128 = pi_u128::<A>();
    let n: u128 = 180;
    let n: u128 = n.checked_mul(scale)?;
    let degree: u128 = muldiv(radian, n, pi)?;
    let degree: B = B::from(degree)?;
    Some(degree)
}

pub fn cast<const A: u8, const B: u8, C>(n: C) -> Option<C> 
where
    C: int::Int,
    precision::Precision<A>: precision::Compatible,
    precision::Precision<B>: precision::Compatible {
    if A == B {
        return Some(n);
    }
    macro_rules! op {
        ($size:ty) => {
            paste::paste! { {
                let old_scale: $size = [<scale_ $size>]::<A>();
                let new_scale: $size = [<scale_ $size>]::<B>();
                let result: $size = n.[<to_ $size>]().unwrap();
                let result: $size = muldiv(result, new_scale, old_scale)?;
                let result: C = C::from(result)?;
                Some(result)
            } }
        };
    }
    if C::IS_SIGNED {
        op!(i128)
    } else {
        op!(u128)
    }
}

pub fn add<T>(x: T, y: T) -> Option<T> where T: int::Int {
    x.checked_add(&y)
}

pub fn sub<T>(x: T, y: T) -> Option<T> where T: int::Int {
    x.checked_sub(&y)
}

pub fn mul<const A: u8, B>(x: B, y: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    macro_rules! op {
        ($size:ty) => {
            paste::paste! { {
                let x: $size = x.[<to_ $size>]().unwrap();
                let y: $size = y.[<to_ $size>]().unwrap();
                let scale: $size = [<scale_ $size>]::<A>();
                let result: $size = muldiv(x, y, scale)?;
                let result: B = B::from(result)?;
                Some(result)
            } }
        };
    }
    if B::IS_SIGNED {
        op!(i128)
    } else {
        op!(u128)
    }
}

pub fn div<const A: u8, B>(x: B, y: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    let scale: u128 = scale_u128::<A>();
    macro_rules! op {
        ($size:ty) => {
            paste::paste! { {
                let x: $size = x.[<to_ $size>]().unwrap();
                let y: $size = y.[<to_ $size>]().unwrap();
                if scale.is_power_of_two() {
                    let z: u32 = scale.trailing_zeros();
                    let result: $size = (x << z).checked_div(y)?;
                    let result: B = B::from(result)?;
                    return Some(result);
                }
                let scale: $size = [<scale_ $size>]::<A>();
                let result: $size = muldiv(x, scale, y)?;
                let result: B = B::from(result)?;
                Some(result)
            } }
        };
    }
    if B::IS_SIGNED {
        op!(i128)
    } else {
        op!(u128)
    }
}

fn muldiv<T>(x: T, y: T, z: T) -> Option<T> where T: int::Int {
    let (a, b) = wide_mul(x, y)?;
    if b >= z {
        return None;
    }
    if b == T::ZERO {
        return Some(a / z);
    }
    Some(fold(a, b, z)? / z)
}

fn fold<T>(x: T, y: T, z: T) -> Option<T> where T: int::Int {
    macro_rules! op {
        ($size:ty) => {
            paste::paste! { {
                let x: $size = x.[<to_ $size>]().unwrap();
                let y: $size = y.[<to_ $size>]().unwrap();
                let z: $size = z.[<to_ $size>]().unwrap();
                let result: $size = (((((y % z) << 64) | (x >> 64)) % z) << 64) | (x & 0xFFFFFFFFFFFFFFFF);
                let result: T = T::from(result)?;
                Some(result)
            } }
        };
    } 
    if T::IS_SIGNED {
        op!(i128)
    } else {
        op!(u128)
    }
}

fn wide_mul<T>(x: T, y: T) -> Option<(T, T)> where T: int::Int {
    macro_rules! op {
        ($size:ty) => {
            paste::paste! { {
                let x: $size = x.[<to_ $size>]().unwrap();
                let y: $size = y.[<to_ $size>]().unwrap();
                let x_hi: $size = x >> 64;
                let x_lo: $size = x & 0xFFFFFFFFFFFFFFFF;
                let y_hi: $size = y >> 64;
                let y_lo: $size = y & 0xFFFFFFFFFFFFFFFF;
                let lo_lo: $size = x_lo * y_lo;
                let lo_hi: $size = x_lo * y_hi;
                let hi_lo: $size = x_hi * y_lo;
                let hi_hi: $size = x_hi * y_hi;
                let m: $size = lo_hi + hi_lo;
                let c: $size = ((m < lo_hi) as $size) << 64;
                let m_lo: $size = m << 64;
                let m_hi: $size = m >> 64;
                let x: $size = lo_lo.wrapping_add(m_lo);
                let y: $size = hi_hi + m_hi + c + ((x < lo_lo) as $size);
                let x: T = T::from(x)?;
                let y: T = T::from(y)?;
                Some((x, y))
            } }
        };
    }
    if T::IS_SIGNED {
        op!(i128)
    } else {
        op!(u128)
    }
}

pub fn sqrt(n: u128) -> u128 {
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

pub fn isqrt_bitwise(mut n: u128) -> u128 {
    let mut res = 0u128;
    let mut bit = 1u128 << 126; // highest even bit below 128
    
    // “bit” starts at the highest power-of-four ≤ 2^126
    while bit > n {
        bit >>= 2;
    }
    while bit != 0 {
        if n >= res + bit {
            n -= res + bit;
            res = (res >> 1) + bit;
        } else {
            res >>= 1;
        }
        bit >>= 2;
    }
    res
}







#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_tan() {
        let x: i128 = 500_0000;
        let x: i128 = tan::<4, _>(x).unwrap();
        assert_eq!(x, -8391);
    }

    #[test]
    pub fn test_sin() {
        let x: i128 = 500_0000;
        let x: i128 = sin::<4, _>(x).unwrap();
        assert_eq!(x, 6428);
    }

    #[test]
    pub fn test_cos() {
        let x: i128 = -500_0000;
        let x: i128 = cos::<4, _>(x).unwrap();
        assert_eq!(x, 9900);
    }
}


