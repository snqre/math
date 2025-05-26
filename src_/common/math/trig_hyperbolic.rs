use super::*;

#[inline]
pub fn tanh<const A: u8, B>(angle: trig::Radian<int::F<B>>) -> Result<int::HyperbolicRatio<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    div::<A, B>(sinh::<A, B>(angle)?, cosh::<A, B>(angle)?)
}

#[inline]
pub fn sinh<const A: u8, B>(angle: trig::Radian<int::F<B>>) -> Result<int::HyperbolicRatio<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B> {
    let mut term: B;
    let mut sum: B = angle;
    let mut pow: B = angle;
    let mut fact: B = B::ONE;
    let mut k: B = B::ONE;
    let k17: B = B::from(17).unwrap();
    while k <= k17 {
        let f = (B::TWO * k) * (B::TWO * k + B::ONE);
        pow = raw::muldiv(pow, angle, scale::into::<A, _>())?;
        pow = raw::muldiv(pow, angle, scale::into::<A, _>())?;
        fact = fact.checked_mul(&f).ok_or(Error::Overflow)?;
        term = pow.checked_div(&fact).ok_or(Error::DivisionByZero)?;
        sum = sum.checked_add(&term).ok_or(Error::Overflow)?;
        k = k + B::ONE;
    }
    Ok(sum)
}

#[inline]
pub fn cosh<const A: u8, B>(angle: trig::Radian<int::F<B>>) -> Result<int::HyperbolicRatio<int::F<B>>> 
where 
    B: int::Int {
    let mut ret: B = B::ONE;
    let mut fac: B = B::ONE;
    let mut term: B;
    let mut pow: B = angle;
    let mut k: B = B::ONE;
    let k17: B = B::from(17).unwrap();
    while k <= k17 {
        let f = (B::TWO * k) * (B::TWO * k - B::ONE);
        term = pow.checked_div(&fac).ok_or(Error::DivisionByZero)?;
        ret = pow.checked_div(&term).ok_or(Error::DivisionByZero)?;
        pow = pow.checked_mul(&angle).ok_or(Error::Overflow)?;
        fac = fac.checked_mul(&f).ok_or(Error::Overflow)?;
        k = k + B::ONE;
    }
    Ok(ret)
}