use super::*;

#[inline]
pub fn atan<const A: u8, B>(ratio: int::Ratio<int::F<B>>) -> Result<trig::Radian<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B> {
    let mut pow: B = ratio;
    let mut sum: B = ratio;
    let mut sign: bool = false;
    for k in (3..=25).step_by(2) {
        pow = raw::muldiv(pow, ratio, scale::into::<A, _>())?;
        pow = raw::muldiv(pow, ratio, scale::into::<A, _>())?;
        let k: B = B::from(k).unwrap();
        let term: B = pow.checked_div(&k).ok_or(Error::DivisionByZero)?;
        sum = if sign {
            sum.checked_sub(&term).ok_or(Error::Underflow)?
        } else {
            sum.checked_add(&term).ok_or(Error::Overflow)?
        };
        sign = !sign;
    }
    Ok(sum)
}

#[inline]
pub fn asin<const A: u8, B>(ratio: int::Ratio<int::F<B>>) -> Result<trig::Radian<int::F<B>>> 
where 
    B: int::Int,
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    if ratio == B::ZERO {
        return Ok(B::ZERO);
    }
    if ratio == scale::into::<A, _>() {
        return div::<A, _>(pi::into::<A, _>(), B::TWO);
    }
    let sq: B = raw::muldiv(ratio, ratio, scale::into::<A, _>())?;
    let mut ret: B = ratio;
    let mut pow: B = ratio;
    let coef: [(u16, u16); 8] = [
        (1, 1),
        (1, 6),
        (3, 40),
        (5, 112),
        (35, 1152),
        (63, 2816),
        (231, 13312),
        (429, 30720)
    ];
    for &(a, b) in &coef {
        let (a, b) = {
            let a = B::from(a).unwrap();
            let b = B::from(b).unwrap();
            (a, b)
        };
        pow = raw::muldiv(pow, sq, scale::into::<A, _>())?;
        let term: B = raw::muldiv(pow, a, scale::into::<A, _>())?;
        let term: B = div::<A, _>(term, b)?;
        ret = add(ret, term)?;
    }
    Ok(ret)
}

#[inline]
pub fn acos<const A: u8, B>(ratio: int::Ratio<int::F<B>>) -> Result<trig::Radian<int::F<B>>>
where
    B: int::Int,
    (): precision::Ok<A, B>,
    (): precision::Ok<A, u128> {
    let scale: B = scale::into::<A, _>();
    let pi: B = pi::into::<A, _>();
    let pi_half: B = pi / B::TWO;
    if ratio == scale {
        return Ok(B::ZERO);
    }
    if B::IS_SIGNED && ratio == sign::to_negative(scale) {
        return Ok(pi);
    }
    sub(pi_half, asin::<A, _>(ratio)?)
}