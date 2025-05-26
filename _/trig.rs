use crate::math;
use crate::precision;
use crate::int;

pub const fn pi_i128<const T: u8>() -> i128 where precision::Precision<T>: precision::Compatible {
    assert(T >= 1);
    assert(T <= 38);
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
            core::hint::unreachable_unchecked()
        }
    }
}

pub const fn pi_u128<const T: u8>() -> u128 where precision::Precision<T>: precision::Compatible {
    assert(T >= 1);
    assert(T <= 38);
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
            core::hint::unreachable_unchecked()
        }
    }
}

pub fn to_rad<const A: u8, B>(degree: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    if B::IS_SIGNED {
        let degree: i128 = degree.to_i128().unwrap();
        let scale: i128 = precision::scale_i128::<A>();
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

fn to_rad_i<const A: u8, B>(degree: B) -> Option<B> where B: int::Int, precision::Precision<A>: precision::Compatible {
    if B::IS_UNSIGNED {
        panic!();
    }
    let degree: i128 = degree.to_i128().unwrap();
    let scale: i128 = math::scale_i128::<A>();
    let pi: i128 = pi_i128::<A>();
    let n: i128 = 180;
    let n: i128 = n.checked_mul(scale)?;
}

fn to_rad_u() {

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