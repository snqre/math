use crate::common::semantic_fixed;

use super::*;

pub trait Handler 
where
    Self: base::Handler {
    #[inline]
    fn tan<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: num::Int {
        let s: B = self.sin::<A, _>(angle)?;
        let c: B = self.cos::<A, _>(angle)?;
        self.div::<A, _>(s, c)
    }
    
    #[inline]
    fn sin<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::Ratio<semantic::Fixed<B>>> 
    where 
        B: num::Int {
        self.cos::<A, _>(self.sub(self.to_radian::<A, _>(deg90::<A, _>()?)?, angle)?)
    }

    #[inline]
    fn cos<const A: u8, B>(&self, angle: int::Radian<int::Fixed<B>>) -> Result<int::Ratio<int::Fixed<B>>> 
    where 
        B: int::Int {
        let scale: B = self.scale::<A, _>();
        let pi: B = self.pi::<A, _>();
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
            term = self.muldiv(term, n, scale)?;
            term = self.muldiv(term, n, scale)?;
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
}

impl<T> Handler for T
where
    T: super::Handler
{}