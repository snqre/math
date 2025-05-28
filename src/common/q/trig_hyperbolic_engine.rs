use super::*;

pub trait TrigHyperbolicEngine 
where
    Self: arithmetic_engine::ArithmeticEngine {
    #[inline]
    fn tanh<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::HyperbolicRatio<B>> where B: int::Int {
        Ok(Self::div::<A, B>(Self::sinh::<A, _>(angle)?, Self::cosh::<A, _>(angle)?)?)
    }
    
    #[inline]
    fn sinh<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::HyperbolicRatio<B>> where B: int::Int {
        let mut term: B;
        let mut sum: B = angle;
        let mut pow: B = angle;
        let mut fact: B = B::N1;
        let mut k: B = B::N1;
        let k17: B = B::from(17).unwrap();
        let scale: B = scale::get::<A, _>();
        while k <= k17 {
            let f = (B::N2 * k) * (B::N2 * k + B::N1);
            pow = Self::muldiv(pow, angle, scale)?;
            pow = Self::muldiv(pow, angle, scale)?;
            fact = fact.checked_mul(&f).ok_or(Error::Overflow)?;
            term = pow.checked_div(&fact).ok_or(Error::DivisionByZero)?;
            sum = sum.checked_add(&term).ok_or(Error::Overflow)?;
            k = k + B::N1;
        }
        Ok(sum)
    }
    
    #[inline]
    fn cosh<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::HyperbolicRatio<B>> where B: int::Int {
        let mut sum = B::N1;
        let mut pow = B::N1;
        let mut fact = B::N1;
    
        let scale = scale::get::<A, B>();
        let angle_sq = Self::muldiv(angle, angle, scale)?;
        let max_terms: u8 = if B::BIT <= 16 { 4 } else if B::BIT <= 32 { 6 } else { 10 };
    
        let threshold = B::from(1).unwrap(); // below 0.01 in Q2

        for n in 1..=max_terms {
            pow = Self::muldiv(pow, angle_sq, scale)?;
            let denom = factorial::<B>(2 * n)?;
            let term = pow.checked_div(&denom).ok_or(Error::DivisionByZero)?;
            
            if term < threshold {
                break;
            }
        
            sum = sum.checked_add(&term).ok_or(Error::Overflow)?;
        }
    
        // Scale result back to fixed-point
        Ok(Self::muldiv(sum, scale, B::N1)?)
    }
}

fn factorial<B: int::Int>(n: u8) -> Result<B> {
    let mut acc = B::N1;
    for i in 2..=n {
        let val = B::from(i).ok_or(Error::Overflow)?;
        acc = acc.checked_mul(&val).ok_or(Error::Overflow)?;
    }
    Ok(acc)
}

impl<T: Engine> TrigHyperbolicEngine for T {}