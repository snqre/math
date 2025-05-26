use super::*;

pub type Ratio<T> = T;

pub type Radian<T> = T;

pub type Degree<T> = T;

#[inline]
pub fn tan<const A: u8, B>(angle: Radian<int::F<B>>) -> Result<Ratio<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    div::<A, _>(sin::<A, _>(angle)?, cos::<A, _>(angle)?)
}

#[inline]
pub fn sin<const A: u8, B>(angle: Radian<int::F<B>>) -> Result<Ratio<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B> {
    cos::<A, _>(sub(trig_conversion::to_radian::<A, _>(deg90::<A, _>()?)?, angle)?)
}

#[inline]
pub fn cos<const A: u8, B>(angle: Radian<int::F<B>>) -> Result<Ratio<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B> {
    let scale: B = scale::into::<A, _>();
    let pi: B = pi::into::<A, _>();
    let pi_two: B = pi.checked_mul(&B::TWO).ok_or(Error::Overflow)?;
    let mut n: B = angle % pi_two;
    if n < B::ZERO {
        n = n.checked_add(&pi_two).ok_or(Error::Overflow)?;
    }
    if n > pi {
        n = n.checked_sub(&pi_two).ok_or(Error::Underflow)?;
    }
    let mut term: B = scale;
    let mut result: B = scale;
    let mut sign: bool = true;
    let mut k: B = B::ONE;
    loop {
        term = raw::muldiv(term, n, scale)?;
        term = raw::muldiv(term, n, scale)?;
        term = term.checked_div(&((B::TWO * k - B::ONE) * (B::TWO * k))).ok_or(Error::DivisionByZero)?;
        if term == B::ZERO {
            break;
        }
        result = if sign {
            result.checked_sub(&term).ok_or(Error::Underflow)?
        } else {
            result.checked_add(&term).ok_or(Error::Overflow)?
        };
        sign = !sign;
        k = k.checked_add(&B::ONE).ok_or(Error::Overflow)?;
    }
    Ok(result)
}

#[inline]
fn deg90<const A: u8, B>() -> Result<Degree<int::F<B>>> where B: int::Int, (): precision::Ok<A, B> {
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
    let out: B = degree.checked_mul(&scale::into::<A, _>()).ok_or(Error::Overflow)?;
    Ok(out)
}