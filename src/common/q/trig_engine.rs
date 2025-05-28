use super::*;

pub trait TrigEngine 
where
    Self: arithmetic_engine::ArithmeticEngine,
    Self: trig_conversion_engine::TrigConversionEngine {
    #[inline]
    fn tan<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> where B: int::Int {
        Ok(Self::div::<A, _>(Self::sin::<A, _>(angle)?, Self::cos::<A, _>(angle)?)?)
    }

    #[inline]
    fn sin<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> where B: int::Int {
        Ok(Self::cos::<A, _>(Self::sub(Self::to_radian::<A, _>(trig::deg90::<A, _>()?)?, angle)?)?)
    }

    #[inline]
    fn cos<const A: u8, B>(angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> where B: int::Int {
        use Error::*;
        
        let (scale, pi, pi_two) = {
            let scale: B = scale::get::<A, _>();
            let pi: B = pi::get::<A, _>();
            let pi_two: B = pi.checked_mul(&B::N2).ok_or(Overflow)?;
            (scale, pi, pi_two)
        };
        let mut n: B = angle % pi_two;
        let () = {
            if n < B::N0 {
                n = n.checked_add(&pi_two).ok_or(Overflow)?;
            }
            if n > pi {
                n = n.checked_sub(&pi_two).ok_or(Underflow)?;
            }
        };
        let mut res: B = scale;
        let () = {
            let mut term: B = scale;
            let mut sign: bool = true;
            let mut k: B = B::N1;
            loop {
                let f: B = (B::N2 * k - B::N1) * (B::N2 * k);
                term = Self::muldiv(term, n, scale)?;
                term = Self::muldiv(term, n, scale)?;
                term = term.checked_div(&f).ok_or(DivisionByZero)?;
                if term == B::N0 {
                    break;
                }
                res = if sign {
                    res.checked_sub(&term).ok_or(Underflow)?
                } else {
                    res.checked_add(&term).ok_or(Overflow)?
                };
                sign = !sign;
                k = k.checked_add(&B::N1).ok_or(Overflow)?;
            }
        };
        Ok(res)
    }
}

impl<T> TrigEngine for T where T: Engine {}