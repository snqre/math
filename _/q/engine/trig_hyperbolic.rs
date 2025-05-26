use super::*;

pub trait Handler {
    #[inline]
    fn tanh<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::HyperbolicRatio<semantic::Fixed<B>>> 
    where
        B: num::Int {
        let s: B = self.sinh::<A, _>(angle)?;
        let c: B = self.cosh::<A, _>(angle)?;
        self.div::<A, B>(s, c)
    }
    
    #[inline]
    fn sinh<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::HyperbolicRatio<semantic::Fixed<B>>> 
    where 
        B: num::Int {
        let mut term: B;
        let mut sum: B = angle;
        let mut pow: B = angle;
        let mut fact: B = B::ONE;
        let mut k: B = B::ONE;
        let k17: B = B::from(17).unwrap();
        let scale: B = self.scale::<A, _>();
        while k <= k17 {
            let f = (B::TWO * k) * (B::TWO * k + B::ONE);
            pow = self.muldiv(pow, angle, scale)?;
            pow = self.muldiv(pow, angle, scale)?;
            fact = fact.checked_mul(&f).ok_or(Error::Overflow)?;
            term = pow.checked_div(&fact).ok_or(Error::DivisionByZero)?;
            sum = sum.checked_add(&term).ok_or(Error::Overflow)?;
            k = k + B::ONE;
        }
        Ok(sum)
    }
    
    #[inline]
    fn cosh<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::HyperbolicRatio<semantic::Fixed<B>>> 
    where 
        B: num::Int {
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
}

impl<T> Handler for T
where
    T: super::Handler
{}