use super::*;

pub use super::Result;
pub use super::Error;

// --- --- ---

pub trait Engine 
where
    Self: Sized,
    Self: Clone,
    Self: Copy,
    Self: TrigHyperbolicEngine,
    Self: TrigReciprocalEngine,
    Self: TrigEngine,
    Self: TrigConversionEngine,
    Self: BaseEngine,
    Self: MuldivEngine,
    Self: SignEngine 
{}

// --- --- ---

pub trait TrigHyperbolicEngine 
where
    Self: BaseEngine {
    #[inline]
    fn tanh<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::HyperbolicRatio<B>> 
    where
        B: int::Int {
        let s: B = self.sinh::<A, _>(angle)?;
        let c: B = self.cosh::<A, _>(angle)?;
        self.div::<A, B>(s, c)
    }
    
    #[inline]
    fn sinh<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::HyperbolicRatio<B>> 
    where 
        B: int::Int {
        let mut term: B;
        let mut sum: B = angle;
        let mut pow: B = angle;
        let mut fact: B = B::N1;
        let mut k: B = B::N1;
        let k17: B = B::from(17).unwrap();
        let scale: B = scale::<A, _>();
        while k <= k17 {
            let f = (B::N2 * k) * (B::N2 * k + B::N1);
            pow = self.muldiv(pow, angle, scale)?;
            pow = self.muldiv(pow, angle, scale)?;
            fact = fact.checked_mul(&f).ok_or(Error::Overflow)?;
            term = pow.checked_div(&fact).ok_or(Error::DivisionByZero)?;
            sum = sum.checked_add(&term).ok_or(Error::Overflow)?;
            k = k + B::N1;
        }
        Ok(sum)
    }
    
    #[inline]
    fn cosh<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::HyperbolicRatio<B>> 
    where 
        B: int::Int {
        let mut ret: B = B::N1;
        let mut fac: B = B::N1;
        let mut term: B;
        let mut pow: B = angle;
        let mut k: B = B::N1;
        let k17: B = B::from(17).unwrap();
        while k <= k17 {
            let f = (B::N2 * k) * (B::N2 * k - B::N1);
            term = pow.checked_div(&fac).ok_or(Error::DivisionByZero)?;
            ret = pow.checked_div(&term).ok_or(Error::DivisionByZero)?;
            pow = pow.checked_mul(&angle).ok_or(Error::Overflow)?;
            fac = fac.checked_mul(&f).ok_or(Error::Overflow)?;
            k = k + B::N1;
        }
        Ok(ret)
    }
}

impl<T: Engine> TrigHyperbolicEngine for T {}

// --- --- ---

pub trait TrigReciprocalEngine 
where
    Self: TrigEngine {
    #[inline]
    fn csc<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let out: B = self.sin::<A, _>(angle)?;
        let out: B = self.div::<A, _>(scale::<A, _>(), out)?;
        Ok(out)
    }
    
    #[inline]
    fn sec<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let out: B = self.cos::<A, _>(angle)?;
        self.div::<A, _>(scale::<A, _>(), out)
    }
    
    #[inline]
    fn cot<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let out: B = self.tan::<A, _>(angle)?;
        self.div::<A, _>(scale::<A, _>(), out)
    }
}

impl<T> TrigReciprocalEngine for T
where
    T: Engine
{}

// --- --- ---

pub trait TrigEngine 
where
    Self: BaseEngine,
    Self: TrigConversionEngine {
    #[inline]
    fn tan<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let s: B = self.sin::<A, _>(angle)?;
        let c: B = self.cos::<A, _>(angle)?;
        self.div::<A, _>(s, c)
    }
    
    #[inline]
    fn sin<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let rad90: B = self.to_radian::<A, _>(deg90::<A, _>()?)?;
        let out: B = self.sub(rad90, angle)?;
        let out: B = self.cos::<A, _>(out)?;
        Ok(out)
    }

    #[inline]
    fn cos<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let scale: B = scale::<A, _>();
        let pi: B = pi::<A, _>();
        let pi_two: B = pi.checked_mul(&B::N2).ok_or(Error::Overflow)?;
        let mut n: B = angle % pi_two;
        if n < B::N0 {
            n = n.checked_add(&pi_two).ok_or(Error::Overflow)?;
        }
        if n > pi {
            n = n.checked_sub(&pi_two).ok_or(Error::Underflow)?;
        }
        let mut term: B = scale;
        let mut result: B = scale;
        let mut sign: bool = true;
        let mut k: B = B::N1;
        loop {
            term = self.muldiv(term, n, scale)?;
            term = self.muldiv(term, n, scale)?;
            term = term.checked_div(&((B::N2 * k - B::N1) * (B::N2 * k))).ok_or(Error::DivisionByZero)?;
            if term == B::N0 {
                break;
            }
            result = if sign {
                result.checked_sub(&term).ok_or(Error::Underflow)?
            } else {
                result.checked_add(&term).ok_or(Error::Overflow)?
            };
            sign = !sign;
            k = k.checked_add(&B::N1).ok_or(Error::Overflow)?;
        }
        Ok(result)
    }
}

#[inline]
fn deg90<const A: u8, B>() -> Result<semantic_fixed::Degree<B>> 
where
    B: int::Int {
    let degree: B = if B::IS_SIGNED {
        let n: i128 = 90;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        n
    } else {
        let n: u128 = 90;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        n
    };
    let out: B = degree.checked_mul(&scale::<A, _>()).ok_or(Error::Overflow)?;
    Ok(out)
}

impl<T> TrigEngine for T
where
    T: Engine
{}

// --- --- ---

pub trait TrigConversionEngine
where
    Self: MuldivEngine {
    #[inline]
    fn to_radian<const A: u8, B>(&self, angle: semantic_fixed::Degree<B>) -> Result<semantic_fixed::Radian<B>> 
    where 
        B: int::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        let scale: B = scale::<A, _>();
        let pi: B = pi::<A, _>();
        self.muldiv(angle, pi, n * scale)
    }
    
    #[inline]
    fn to_degree<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Degree<B>> 
    where 
        B: int::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        let scale: B = scale::<A, _>();
        let pi: B = pi::<A, _>();
        self.muldiv(angle, n * scale, pi)
    }
}

#[inline]
fn pi<const A: u8, B>() -> B 
where 
    B: int::Int {
    if B::IS_SIGNED {
        let pi: i128 = signed_pi::<A>();
        let pi: B = unsafe {
            B::from(pi).unwrap_unchecked()
        };
        return pi;
    }
    let pi: u128 = unsigned_pi::<A>();
    let pi: B = unsafe {
        B::from(pi).unwrap_unchecked()
    };
    pi
}

const fn signed_pi<const T: u8>() -> i128 {
    assert!(T != 0);
    assert!(T <= 37);
    match T {
        1 => 31,
        2 => 314,
        3 => 3141,
        4 => 31415,
        5 => 314159,
        6 => 3141592,
        7 => 31415926,
        8 => 314159265,
        9 => 3141592653,
        10 => 31415926535,
        11 => 314159265358,
        12 => 3141592653589,
        13 => 31415926535897,
        14 => 314159265358979,
        15 => 3141592653589793,
        16 => 31415926535897932,
        17 => 314159265358979323,
        18 => 3141592653589793238,
        19 => 31415926535897932384,
        20 => 314159265358979323846,
        21 => 3141592653589793238462,
        22 => 31415926535897932384626,
        23 => 314159265358979323846264,
        24 => 3141592653589793238462643,
        25 => 31415926535897932384626433,
        26 => 314159265358979323846264338,
        27 => 3141592653589793238462643383,
        28 => 31415926535897932384626433832,
        29 => 314159265358979323846264338327,
        30 => 3141592653589793238462643383279,
        31 => 31415926535897932384626433832795,
        32 => 314159265358979323846264338327950,
        33 => 3141592653589793238462643383279502,
        34 => 31415926535897932384626433832795028,
        35 => 314159265358979323846264338327950288,
        36 => 3141592653589793238462643383279502884,
        37 => 31415926535897932384626433832795028841,
        _ => panic!()
    }
}

const fn unsigned_pi<const T: u8>() -> u128 {
    match T {
        1 => 31,
        2 => 314,
        3 => 3141,
        4 => 31415,
        5 => 314159,
        6 => 3141592,
        7 => 31415926,
        8 => 314159265,
        9 => 3141592653,
        10 => 31415926535,
        11 => 314159265358,
        12 => 3141592653589,
        13 => 31415926535897,
        14 => 314159265358979,
        15 => 3141592653589793,
        16 => 31415926535897932,
        17 => 314159265358979323,
        18 => 3141592653589793238,
        19 => 31415926535897932384,
        20 => 314159265358979323846,
        21 => 3141592653589793238462,
        22 => 31415926535897932384626,
        23 => 314159265358979323846264,
        24 => 3141592653589793238462643,
        25 => 31415926535897932384626433,
        26 => 314159265358979323846264338,
        27 => 3141592653589793238462643383,
        28 => 31415926535897932384626433832,
        29 => 314159265358979323846264338327,
        30 => 3141592653589793238462643383279,
        31 => 31415926535897932384626433832795,
        32 => 314159265358979323846264338327950,
        33 => 3141592653589793238462643383279502,
        34 => 31415926535897932384626433832795028,
        35 => 314159265358979323846264338327950288,
        36 => 3141592653589793238462643383279502884,
        37 => 31415926535897932384626433832795028841,
        38 => 314159265358979323846264338327950288419,
        _ => unsafe {
            ::core::hint::unreachable_unchecked()
        }
    }
}

impl<T> TrigConversionEngine for T
where
    T: Engine
{}

// --- --- ---

pub trait BaseEngine 
where
    Self: MuldivEngine {
    #[inline]
    fn add<T>(&self, x: semantic_fixed::Num<T>, y: semantic_fixed::Num<T>) -> Result<semantic_fixed::Num<T>>
    where
        T: int::Int {
        x.checked_add(&y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(&self, x: semantic_fixed::Num<T>, y: semantic_fixed::Num<T>) -> Result<semantic_fixed::Num<T>>
    where
        T: int::Int {
        x.checked_sub(&y).ok_or(Error::Overflow)
    }

    #[inline]
    fn mul<const A: u8, B>(&self, x: semantic_fixed::Num<B>, y: semantic_fixed::Num<B>) -> Result<semantic_fixed::Num<B>>
    where
        B: int::Int {
        self.muldiv(x, y, scale::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(&self, x: semantic_fixed::Num<B>, y: semantic_fixed::Num<B>) -> Result<semantic_fixed::Num<B>> 
    where 
        B: int::Int {
        let scale: u128 = scale::<A, u128>();
        if scale.is_power_of_two() {
            let ret: B = x << scale.trailing_zeros().try_into().unwrap();
            return Ok(ret)
        }
        let scale: B = unsafe {
            B::from(scale).unwrap_unchecked()
        };
        self.muldiv(x, scale, y)
    }
}

#[inline]
fn scale<const A: u8, B>() -> B 
where 
    B: int::Int {
    if B::IS_SIGNED {
        let scale: i128 = signed_scale::<A>();
        let scale: B = unsafe {
            B::from(scale).unwrap_unchecked()
        };
        return scale;
    }
    let scale: u128 = unsigned_scale::<A>();
    let scale: B = unsafe {
        B::from(scale).unwrap_unchecked()
    };
    scale
}

const fn signed_scale<const T: u8>() -> i128 {
    const BASE: i128 = 10;
    match T {
        1 => BASE.pow(1),
        2 => BASE.pow(2),
        3 => BASE.pow(3),
        4 => BASE.pow(4),
        5 => BASE.pow(5),
        6 => BASE.pow(6),
        7 => BASE.pow(7),
        8 => BASE.pow(8),
        9 => BASE.pow(9),
        10 => BASE.pow(10),
        11 => BASE.pow(11),
        12 => BASE.pow(12),
        13 => BASE.pow(13),
        14 => BASE.pow(14),
        15 => BASE.pow(15),
        16 => BASE.pow(16),
        17 => BASE.pow(17),
        18 => BASE.pow(18),
        19 => BASE.pow(19),
        20 => BASE.pow(20),
        21 => BASE.pow(21),
        22 => BASE.pow(22),
        23 => BASE.pow(23),
        24 => BASE.pow(24),
        25 => BASE.pow(25),
        26 => BASE.pow(26),
        27 => BASE.pow(27),
        28 => BASE.pow(28),
        29 => BASE.pow(29),
        30 => BASE.pow(30),
        31 => BASE.pow(31),
        32 => BASE.pow(32),
        33 => BASE.pow(33),
        34 => BASE.pow(34),
        35 => BASE.pow(35),
        36 => BASE.pow(36),
        37 => BASE.pow(37),
        38 => BASE.pow(38),
        _ => panic!()
    }
}

const fn unsigned_scale<const T: u8>() -> u128 {
    const BASE: u128 = 10;
    match T {
        1 => BASE.pow(1),
        2 => BASE.pow(2),
        3 => BASE.pow(3),
        4 => BASE.pow(4),
        5 => BASE.pow(5),
        6 => BASE.pow(6),
        7 => BASE.pow(7),
        8 => BASE.pow(8),
        9 => BASE.pow(9),
        10 => BASE.pow(10),
        11 => BASE.pow(11),
        12 => BASE.pow(12),
        13 => BASE.pow(13),
        14 => BASE.pow(14),
        15 => BASE.pow(15),
        16 => BASE.pow(16),
        17 => BASE.pow(17),
        18 => BASE.pow(18),
        19 => BASE.pow(19),
        20 => BASE.pow(20),
        21 => BASE.pow(21),
        22 => BASE.pow(22),
        23 => BASE.pow(23),
        24 => BASE.pow(24),
        25 => BASE.pow(25),
        26 => BASE.pow(26),
        27 => BASE.pow(27),
        28 => BASE.pow(28),
        29 => BASE.pow(29),
        30 => BASE.pow(30),
        31 => BASE.pow(31),
        32 => BASE.pow(32),
        33 => BASE.pow(33),
        34 => BASE.pow(34),
        35 => BASE.pow(35),
        36 => BASE.pow(36),
        37 => BASE.pow(37),
        38 => BASE.pow(38),
        _ => panic!()
    }
}

impl<T> BaseEngine for T
where
    T: Engine
{}

// --- --- ---

pub trait MuldivEngine {
    #[inline]
    fn muldiv<T>(&self, x: T, y: T, z: T) -> Result<T> 
    where
        T: int::Int {
        if z == T::N0 {
            return Err(Error::DivisionByZero);
        }
        match (T::BIT, T::IS_SIGNED) {
            (_, true) if T::BIT <= 64 => {
                let ret: T = x.checked_mul(&y).ok_or(Error::Overflow)?;
                let ret: T = ret / z;
                Ok(ret)
            },
            (_, false) if T::BIT < 128 => {
                let (a, b) = wide_mul(x, y)?;
                if b >= z {
                    return Err(Error::Overflow);
                }
                if b == T::N0 {
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
    T: int::Int {
    if T::IS_SIGNED {
        return signed_fold(x, y, z);
    }
    unsigned_fold(x, y, z)
}

#[inline]
fn signed_fold<T>(x: T, y: T, z: T) -> Result<T> 
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
        return Err(Error::Overflow);
    }
    if ret < T::MIN_I128 {
        return Err(Error::Underflow);
    }
    let ret: T = unsafe {
        T::from(ret).unwrap_unchecked()
    };
    Ok(ret)
}

#[inline]
fn unsigned_fold<T>(x: T, y: T, z: T) -> Result<T> 
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
        return Err(Error::Overflow)
    }
    if r < T::MIN_U128 {
        return Err(Error::Underflow)
    }
    let r: T = unsafe {
        T::from(r).unwrap_unchecked()
    };
    Ok(r)
}

#[inline]
fn wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
where 
    T: int::Int {
    if T::IS_SIGNED {
        return signed_wide_mul(x, y);
    }
    unsigned_wide_mul(x, y)
}

#[inline]
fn signed_wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
where 
    T: int::Int {
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
        let lo_lo: T = x_lo.checked_mul(&y_lo).ok_or(Error::Overflow)?;
        let lo_hi: T = x_lo.checked_mul(&y_hi).ok_or(Error::Overflow)?;
        let hi_lo: T = x_hi.checked_mul(&y_lo).ok_or(Error::Overflow)?;
        let hi_hi: T = x_hi.checked_mul(&y_hi).ok_or(Error::Overflow)?;
        (lo_lo, lo_hi, hi_lo, hi_hi)
    };
    let a: T = lo_hi.checked_add(&hi_lo).ok_or(Error::Overflow)?;
    let b: T = a << n;
    let hi: T = if lo_lo > lo_lo.wrapping_add(&b) {
        T::N1
    } else {
        T::N0
    };
    let hi: T = hi_hi
        .checked_add(&(a >> n))
        .ok_or(Error::Overflow)?
        .checked_add(&hi)
        .ok_or(Error::Overflow)?;
    let lo: T = lo_lo.wrapping_add(&b);
    Ok((lo, hi))
}

#[inline]
fn unsigned_wide_mul<T>(x: T, y: T) -> Result<(T, T)> 
where 
    T: int::Int, {
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
        return Err(Error::Overflow);
    }
    if a < T::MIN_U128 {
        return Err(Error::Underflow);
    }
    if b > T::MAX_U128 {
        return Err(Error::Overflow);
    }
    if b < T::MIN_U128 {
        return Err(Error::Underflow);
    }
    let (a, b) = unsafe {
        let a: T = T::from(a).unwrap_unchecked();
        let b: T = T::from(b).unwrap_unchecked();
        (a, b)
    };
    Ok((a, b))
}

impl<T> MuldivEngine for T
where
    T: Engine
{}

// --- --- ---

pub trait SignEngine {
    #[inline]
    fn to_negative<T>(n: T) -> T 
    where 
        T: int::Int {
        if n == T::N0 {
            return T::N0;
        }
        T::N0 - n
    }
    
    #[inline]
    fn to_positive<T>(n: T) -> T 
    where 
        T: int::Int {
        if n == T::N0 {
            return T::N0;
        }
        if n > T::N0 {
            return n;
        }
        T::N0 - n
    }
}

impl<T> SignEngine for T 
where
    T: Engine 
{}

// --- --- ---

#[derive(Debug, Clone, Copy)]
pub struct DefaultEngine;

impl Engine for DefaultEngine {}

// --- --- ---

macro_rules! ok {
    ($($size:ty => $precision:expr)*) => {
        pub trait Ok<const A: u8, B> {}
        $(
            impl Ok<$precision, $size> for () {}
        )*
    };
} 

ok! {
    // `0` not compatible because it is redundant.

    u8 => 1
    u8 => 2

    u16 => 1
    u16 => 2
    u16 => 3
    u16 => 4

    u32 => 1
    u32 => 2
    u32 => 3
    u32 => 4
    u32 => 5
    u32 => 6
    u32 => 7
    u32 => 8
    u32 => 9

    u64 => 1
    u64 => 2
    u64 => 3
    u64 => 4
    u64 => 5
    u64 => 6
    u64 => 7
    u64 => 8
    u64 => 9
    u64 => 10
    u64 => 11
    u64 => 12
    u64 => 13
    u64 => 14
    u64 => 15
    u64 => 16
    u64 => 17
    u64 => 18
    u64 => 19

    u128 => 1
    u128 => 2
    u128 => 3
    u128 => 4
    u128 => 5
    u128 => 6
    u128 => 7
    u128 => 8
    u128 => 9
    u128 => 10
    u128 => 11
    u128 => 12
    u128 => 13
    u128 => 14
    u128 => 15
    u128 => 16
    u128 => 17
    u128 => 18
    u128 => 19
    u128 => 20
    u128 => 21
    u128 => 22
    u128 => 23
    u128 => 24
    u128 => 25
    u128 => 26
    u128 => 27
    u128 => 28
    u128 => 29
    u128 => 30
    u128 => 31
    u128 => 32
    u128 => 33
    u128 => 34
    u128 => 35
    u128 => 36
    u128 => 37
    u128 => 38

    // `i8` not compatible with `180` which is required by `trig_conversion`.
    
    i16 => 1
    i16 => 2
    i16 => 3
    i16 => 4

    i32 => 1
    i32 => 2
    i32 => 3
    i32 => 4
    i32 => 5
    i32 => 6
    i32 => 7
    i32 => 8
    i32 => 9

    i64 => 1
    i64 => 2
    i64 => 3
    i64 => 4
    i64 => 5
    i64 => 6
    i64 => 7
    i64 => 8
    i64 => 9
    i64 => 10
    i64 => 11
    i64 => 12
    i64 => 13
    i64 => 14
    i64 => 15
    i64 => 16
    i64 => 17
    i64 => 18

    i128 => 1
    i128 => 2
    i128 => 3
    i128 => 4
    i128 => 5
    i128 => 6
    i128 => 7
    i128 => 8
    i128 => 9
    i128 => 10
    i128 => 11
    i128 => 12
    i128 => 13
    i128 => 14
    i128 => 15
    i128 => 16
    i128 => 17
    i128 => 18
    i128 => 19
    i128 => 20
    i128 => 21
    i128 => 22
    i128 => 23
    i128 => 24
    i128 => 25
    i128 => 26
    i128 => 27
    i128 => 28
    i128 => 29
    i128 => 30
    i128 => 31
    i128 => 32
    i128 => 33
    i128 => 34
    i128 => 35
    i128 => 36
    i128 => 37

    // `i128 => 38` not compatible with `pi` due to potential overflow.
}

// --- --- ---

pub type Q1U8 = Q1<u8>;
pub type Q2U8 = Q2<u8>;

pub type Q1U16 = Q1<u16>;
pub type Q2U16 = Q2<u16>;
pub type Q3U16 = Q3<u16>;
pub type Q4U16 = Q4<u16>;

pub type Q1U32 = Q1<u32>;
pub type Q2U32 = Q2<u32>;
pub type Q3U32 = Q3<u32>;
pub type Q4U32 = Q4<u32>;
pub type Q5U32 = Q5<u32>;
pub type Q6U32 = Q6<u32>;
pub type Q7U32 = Q7<u32>;
pub type Q8U32 = Q8<u32>;
pub type Q9U32 = Q9<u32>;

pub type Q1U64 = Q1<u64>;
pub type Q2U64 = Q2<u64>;
pub type Q3U64 = Q3<u64>;
pub type Q4U64 = Q4<u64>;
pub type Q5U64 = Q5<u64>;
pub type Q6U64 = Q6<u64>;
pub type Q7U64 = Q7<u64>;
pub type Q8U64 = Q8<u64>;
pub type Q9U64 = Q9<u64>;
pub type Q10U64 = Q10<u64>;
pub type Q11U64 = Q11<u64>;
pub type Q12U64 = Q12<u64>;

pub type Q1U128 = Q1<u128>;
pub type Q2U128 = Q2<u128>;
pub type Q3U128 = Q3<u128>;
pub type Q4U128 = Q4<u128>;
pub type Q5U128 = Q5<u128>;
pub type Q6U128 = Q6<u128>;
pub type Q7U128 = Q7<u128>;
pub type Q8U128 = Q8<u128>;
pub type Q9U128 = Q9<u128>;
pub type Q10U128 = Q10<u128>;
pub type Q11U128 = Q11<u128>;
pub type Q12U128 = Q12<u128>;
pub type Q13U128 = Q13<u128>;
pub type Q14U128 = Q14<u128>;
pub type Q15U128 = Q15<u128>;
pub type Q16U128 = Q16<u128>;
pub type Q17U128 = Q17<u128>;
pub type Q18U128 = Q18<u128>;
pub type Q19U128 = Q19<u128>;
pub type Q20U128 = Q20<u128>;
pub type Q21U128 = Q21<u128>;
pub type Q22U128 = Q22<u128>;
pub type Q23U128 = Q23<u128>;
pub type Q24U128 = Q24<u128>;
pub type Q25U128 = Q25<u128>;
pub type Q26U128 = Q26<u128>;
pub type Q27U128 = Q27<u128>;
pub type Q28U128 = Q28<u128>;
pub type Q29U128 = Q29<u128>;
pub type Q30U128 = Q30<u128>;
pub type Q31U128 = Q31<u128>;
pub type Q32U128 = Q32<u128>;
pub type Q33U128 = Q33<u128>;
pub type Q34U128 = Q34<u128>;
pub type Q35U128 = Q35<u128>;
pub type Q36U128 = Q36<u128>;
pub type Q37U128 = Q37<u128>;
pub type Q38U128 = Q38<u128>;

pub type Q1I16 = Q1<i16>;
pub type Q2I16 = Q2<i16>;
pub type Q3I16 = Q3<i16>;
pub type Q4I16 = Q4<i16>;

// --- --- ---

pub type Q1<T> = Default<1, T>;
pub type Q2<T> = Default<2, T>;
pub type Q3<T> = Default<3, T>;
pub type Q4<T> = Default<4, T>;
pub type Q5<T> = Default<5, T>;
pub type Q6<T> = Default<6, T>;
pub type Q7<T> = Default<7, T>;
pub type Q8<T> = Default<8, T>;
pub type Q9<T> = Default<9, T>;
pub type Q10<T> = Default<10, T>;
pub type Q11<T> = Default<11, T>;
pub type Q12<T> = Default<12, T>;
pub type Q13<T> = Default<13, T>;
pub type Q14<T> = Default<14, T>;
pub type Q15<T> = Default<15, T>;
pub type Q16<T> = Default<16, T>;
pub type Q17<T> = Default<17, T>;
pub type Q18<T> = Default<18, T>;
pub type Q19<T> = Default<19, T>;
pub type Q20<T> = Default<20, T>;
pub type Q21<T> = Default<21, T>;
pub type Q22<T> = Default<22, T>;
pub type Q23<T> = Default<23, T>;
pub type Q24<T> = Default<24, T>;
pub type Q25<T> = Default<25, T>;
pub type Q26<T> = Default<26, T>;
pub type Q27<T> = Default<27, T>;
pub type Q28<T> = Default<28, T>;
pub type Q29<T> = Default<29, T>;
pub type Q30<T> = Default<30, T>;
pub type Q31<T> = Default<31, T>;
pub type Q32<T> = Default<32, T>;
pub type Q33<T> = Default<33, T>;
pub type Q34<T> = Default<34, T>;
pub type Q35<T> = Default<35, T>;
pub type Q36<T> = Default<36, T>;
pub type Q37<T> = Default<37, T>;
pub type Q38<T> = Default<38, T>;

pub type Default<const A: u8, B> = Q<A, B, DefaultEngine>;

// --- --- ---

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B, C> 
where
    B: int::Int,
    C: Engine {
    v: B,
    engine: C
}

// --- --- ---

#[inline]
pub fn new<const A: u8, B>(v: B) -> Default<A, B> 
where
    B: int::Int {
    Q::new(v)
}

impl<const A: u8, B> Default<A, B>
where
    B: int::Int {
    #[inline]
    pub fn new(v: B) -> Self {
        Self::custom(v, DefaultEngine)
    }
}

// --- --- ---

#[inline]
pub fn custom<const A: u8, B, C>(v: B, engine: C) -> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    Q::custom(v, engine)
}

impl<const A: u8, B, C> Q<A, B, C> 
where
    B: int::Int,
    C: Engine {
    #[inline]
    pub fn custom(v: B, engine: C) -> Self {
        Self { v, engine }
    }
}

// --- --- ---

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    pub fn csc(&self) -> Result<semantic::Ratio<Self>> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.csc::<A, _>(out)?;
        Ok(custom(out, engine))
    }

    pub fn sec(&self) -> Result<semantic::Ratio<Self>> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.sec::<A, _>(out)?;
        Ok(custom(out, engine))
    }

    pub fn cot(&self) -> Result<semantic::Ratio<Self>> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.cot::<A, _>(out)?;
        Ok(custom(out, engine))
    }
}

// --- --- ---

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine,
    (): Ok<A, B> {
    pub fn tan(&self) -> Result<semantic::Ratio<Self>> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.tan::<A, _>(out)?;
        Ok(custom(out, engine))
    }

    pub fn sin(&self) -> Result<semantic::Ratio<Self>> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.sin::<A, _>(out)?;
        Ok(custom(out, engine))
    }

    pub fn cos(&self) -> Result<semantic::Ratio<Self>> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.cos::<A, _>(out)?;
        Ok(custom(out, engine))
    }
}

// --- --- ---

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine,
    (): Ok<A, B> {
    pub fn to_radian(&self) -> Result<Self> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.to_radian::<A, _>(out)?;
        Ok(custom(out, engine))
    }

    pub fn to_degree(&self) -> Result<Self> {
        let engine: C = self.engine;
        let out: B = self.v;
        let out: B = engine.to_degree::<A, _>(out)?;
        Ok(custom(out, engine))
    }
}

// --- --- ---

impl<const A: u8, B, C> ::core::ops::Add for Q<A, B, C>
where
    B: int::Int,
    C: Engine,
    (): precision::Ok<A, B> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let engine: C = self.engine;
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = engine.add(x, y)?;
        Ok(custom(out, engine))
    }
}

// --- --- ---

impl<const A: u8, B, C> ::core::ops::Sub for Q<A, B, C>
where
    B: int::Int,
    C: Engine,
    (): precision::Ok<A, B> {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let engine: C = self.engine;
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = engine.sub(x, y)?;
        Ok(custom(out, engine))
    }
}

// --- --- ---

impl<const A: u8, B, C> ::core::ops::Mul for Q<A, B, C>
where
    B: int::Int,
    C: Engine,
    (): Ok<A, B> {
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let engine: C = self.engine;
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = engine.mul::<A, _>(x, y)?;
        Ok(custom(out, engine))
    }
}

// --- --- ---

impl<const A: u8, B, C> ::core::ops::Div for Q<A, B, C>
where
    B: int::Int,
    C: Engine,
    (): Ok<A, B> {
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let engine: C = self.engine;
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = engine.div::<A, _>(x, y)?;
        Ok(Q::custom(out, engine))
    }
}

// --- --- ---

impl<const A: u8, B, C> Ord for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized {
        if self < min {
            return min;
        }
        if self > max {
            return max;
        }
        self
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized {
        if self < other {
            return other;
        }
        self
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized {
        if self > other {
            return other;
        }
        self
    }

    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        if self < other {
            return ::core::cmp::Ordering::Less;
        }
        if self > other {
            return ::core::cmp::Ordering::Greater;
        }
        ::core::cmp::Ordering::Equal
    }
}

// --- --- ---

impl<const A: u8, B, C> PartialOrd for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    fn ge(&self, other: &Self) -> bool {
        let x: B = self.v;
        let y: B = other.v;
        x >= y
    }

    fn gt(&self, other: &Self) -> bool {
        let x: B = self.v;
        let y: B = other.v;
        x > y
    }

    fn le(&self, other: &Self) -> bool {
        let x: B = self.v;
        let y: B = other.v;
        x <= y
    }

    fn lt(&self, other: &Self) -> bool {
        let x: B = self.v;
        let y: B = other.v;
        x < y
    }

    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// --- --- ---

impl<const A: u8, B, C> Eq for Q<A, B, C>
where
    B: int::Int,
    C: Engine
{}

// --- --- ---

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    fn eq(&self, other: &Self) -> bool {
        let x: B = self.v;
        let y: B = other.v;
        x == y
    }
}

// --- --- ---

#[allow(clippy::zero_prefixed_literal)]
#[cfg(test)]
mod test {
    use super::*;

    // --- --- ---

    #[::rstest::rstest]
    #[case(1_00, 1_00, 2_00)]
    fn add<T>(#[case] x: T, #[case] y: T, #[case] ok: T) 
    where
        T: ::core::fmt::Debug,
        T: int::Int,
        (): precision::Ok<2, T> {
        let x: Q2<T> = x.into();
        let y: Q2<T> = y.into();
        let result: Q2<T> = (x + y).unwrap();
        assert_eq!(result, new(ok));
    }

    #[::rstest::rstest]
    #[case(1_00, 1_00, 0_00)]
    fn sub<T>(#[case] x: T, #[case] y: T, #[case] ok: T) 
    where
        T: ::core::fmt::Debug,
        T: int::Int,
        (): precision::Ok<2, T> {
        let x: Q2<T> = x.into();
        let y: Q2<T> = y.into();
        let result: Q2<T> = (x - y).unwrap();
        assert_eq!(result, new(ok));
    }
    
    #[::rstest::rstest]
    #[case(1_00, 1_00, 1_00)]
    fn mul<T>(#[case] x: T, #[case] y: T, #[case] ok: T) 
    where
        T: ::core::fmt::Debug,
        T: int::Int,
        (): Ok<2, T> {
        let x: Q2<T> = x.into();
        let y: Q2<T> = y.into();
        let result: Q2<T> = (x * y).unwrap();
        assert_eq!(result, new(ok));
    }

    #[::rstest::rstest]
    #[case(1_00, 1_00, 1_00)]
    #[case(7_25, 0_50, 14_50)]
    fn div<T>(#[case] x: T, #[case] y: T, #[case] ok: T)
    where
        T: ::core::fmt::Debug,
        T: int::Int,
        (): Ok<2, T> {
        let x: Q2<T> = x.into();
        let y: Q2<T> = y.into();
        let result: Q2<T> = (x / y).unwrap();
        assert_eq!(result, new(ok));
    }
}