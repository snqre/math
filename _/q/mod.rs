use crate::common::num;
use crate::common::semantic;
use crate::common::semantic_fixed;
use crate::common::pi;
use crate::common::scale;

pub trait Engine: Clone + Copy {

    #[inline]
    fn tanh<const A: u8, B: Int>(&self, angle: Radian<Fixed<B>>) -> Result<HyperbolicRatio<Fixed<B>>> {
        let s: B = self.sinh::<A, _>(angle)?;
        let c: B = self.cosh::<A, _>(angle)?;
        self.div::<A, B>(s, c)
    }
    
    #[inline]
    fn sinh<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::HyperbolicRatio<int::F<B>>> 
    where 
        B: int::Int {
        let mut term: B;
        let mut sum: B = angle;
        let mut pow: B = angle;
        let mut fact: B = B::ONE;
        let mut k: B = B::ONE;
        let k17: B = B::from(17).unwrap();
        while k <= k17 {
            let f = (B::TWO * k) * (B::TWO * k + B::ONE);
            pow = self.muldiv(pow, angle, scale::into::<A, _>())?;
            pow = self.muldiv(pow, angle, scale::into::<A, _>())?;
            fact = fact.checked_mul(&f).ok_or(Error::Overflow)?;
            term = pow.checked_div(&fact).ok_or(Error::DivisionByZero)?;
            sum = sum.checked_add(&term).ok_or(Error::Overflow)?;
            k = k + B::ONE;
        }
        Ok(sum)
    }
    
    #[inline]
    fn cosh<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::HyperbolicRatio<int::F<B>>> 
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

    #[inline]
    fn csc<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
    where 
        B: int::Int {
        let out: B = self.sin::<A, _>(angle)?;
        self.inv::<A, _>(out)
    }
    
    #[inline]
    fn sec<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
    where 
        B: int::Int {
        let out: B = self.cos::<A, _>(angle)?;
        self.inv::<A, _>(out)
    }
    
    #[inline]
    fn cot<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
    where 
        B: int::Int {
        let out: B = self.tan::<A, _>(angle)?;
        self.inv::<A, _>(out)
    }

    #[inline]
    fn inv<const A: u8, B>(&self, n: B) -> Result<B> 
    where
        B: int::Int {
        self.div::<A, _>(scale::<A, _>(), n)
    }

    #[inline]
    fn tan<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
    where 
        B: int::Int {
        let s: B = self.sin::<A, _>(angle)?;
        let c: B = self.cos::<A, _>(angle)?;
        self.div::<A, _>(s, c)
    }
    
    #[inline]
    fn sin<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
    where 
        B: int::Int {
        self.cos::<A, _>(self.sub(self.to_radian::<A, _>(deg90::<A, _>()?)?, angle)?)
    }

    #[inline]
    fn cos<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::Ratio<int::F<B>>> 
    where 
        B: int::Int {
        let scale: B = scale::<A, _>();
        let pi: B = π::<A, _>();
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

    #[inline]
    fn to_radian<const A: u8, B>(&self, angle: int::Degree<int::F<B>>) -> Result<int::Radian<int::F<B>>> 
    where 
        B: int::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        self.muldiv(angle, π::<A, _>(), n * scale::<A, _>())
    }
    
    #[inline]
    fn to_degree<const A: u8, B>(&self, angle: int::Radian<int::F<B>>) -> Result<int::Degree<int::F<B>>> 
    where 
        B: int::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        self.muldiv(angle, n * scale::<A, _>(), π::<A, _>())
    }

    #[inline]
    fn to_negative<T>(&self, n: T) -> T 
    where 
        T: int::Int {
        if n == T::ZERO {
            return T::ZERO;
        }
        T::ZERO - n
    }
    
    #[inline]
    fn to_positive<T>(&self, n: T) -> T 
    where 
        T: int::Int {
        if n == T::ZERO {
            return T::ZERO;
        }
        if n > T::ZERO {
            return n;
        }
        T::ZERO - n
    }

    #[inline] // muldiv n, n, scale ??
    fn sqrt<T>(&self, n: int::F<T>) -> int::F<T> 
    where 
        T: int::Int {
        if n == T::ZERO {
            return T::ZERO;
        }
        let mut x_0 = n / T::TWO;
        let mut x_1 = (x_0 + n / x_0) / T::TWO;
        while x_1 < x_0 {
            x_0 = x_1;
            x_1 = (x_0 + n / x_0) / T::TWO;
        }
        x_0
    }

    #[inline]
    fn add<T>(&self, x: int::F<T>, y: int::F<T>) -> Result<int::F<T>> 
    where 
        T: int::Int {
        x.checked_add(&y).ok_or(Error::Overflow)
    }
    
    #[inline]
    fn sub<T>(&self, x: int::F<T>, y: int::F<T>) -> Result<int::F<T>> 
    where 
        T: int::Int {
        x.checked_sub(&y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(&self, x: int::F<B>, y: int::F<B>) -> Result<int::F<B>> 
    where 
        B: int::Int {
        self.muldiv(x, y, scale::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(&self, x: int::F<B>, y: int::F<B>) -> Result<int::F<B>> 
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

    #[inline]
    fn muldiv<T: Int>(&self, x: T, y: T, z: T) -> Result<T> {
        if z == T::ZERO {
            return Err(DivisionByZero);
        }
        match (T::BITS_USIZE, T::IS_SIGNED) {
            (_, true) if T::BITS_USIZE <= 64 => {
                let ret: T = x.checked_mul(&y).ok_or(Overflow)?;
                let ret: T = ret / z;
                Ok(ret)
            },
            (_, false) if T::BITS_USIZE < 128 => {
                let (a, b) = muldiv::wide_mul(x, y)?;
                if b >= z {
                    return Err(Error::Overflow);
                }
                if b == T::ZERO {
                    return Ok(a / z);
                }
                Ok(muldiv::fold(a, b, z)? / z)
            },
            (128, _) => {
                let ret: T = x.checked_mul(&y).ok_or(Overflow)?;
                Ok(ret / z)
            },
            _ => unsafe {
                ::core::hint::unreachable_unchecked();
            }
        }
    }
}

#[inline]
fn deg90<const A: u8, B: Int>() -> Result<Degree<Fixed<B>>> {
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

















pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

pub type Default<const A: u8, B> = Q<A, B, engine::Default>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B: int::Int, C: engine::Handler> {
    v: B,
    engine: C
}

pub mod engine;

mod impls;
mod default_engine;
mod muldiv;
mod pi;
mod scale;
mod trig;
mod variant_generic;
mod variant;