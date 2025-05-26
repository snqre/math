use super::*;


// --- --- ---

pub trait Engine
where
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

pub trait TrigHyperbolicEngine {
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
    fn cosh<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::HyperbolicRatio<B>> 
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
}

impl<T> TrigHyperbolicEngine for T
where
    T: Engine
{}


// --- --- ---

pub trait TrigReciprocalEngine 
where
    Self: TrigEngine {
    #[inline]
    fn csc<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let out: B = self.sin::<A, _>(angle)?;
        let out: B = self.div::<A, _>(scale::into::<A, _>(), out)?;
        Ok(out)
    }
    
    #[inline]
    fn sec<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let out: B = self.cos::<A, _>(angle)?;
        self.div::<A, _>(scale::into::<A, _>(), out)
    }
    
    #[inline]
    fn cot<const A: u8, B>(&self, angle: semantic_fixed::Radian<B>) -> Result<semantic_fixed::Ratio<B>> 
    where 
        B: int::Int {
        let out: B = self.tan::<A, _>(angle)?;
        self.div::<A, _>(scale::into::<A, _>(), out)
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
        let scale: B = scale::into::<A, _>();
        let pi: B = pi::into::<A, _>();
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
    let out: B = degree.checked_mul(&scale::into::<A, _>()).ok_or(Error::Overflow)?;
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
        let scale: B = scale::into::<A, _>();
        let pi: B = pi::into::<A, _>();
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
        let scale: B = scale::into::<A, _>();
        let pi: B = pi::into::<A, _>();
        self.muldiv(angle, n * scale, pi)
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
        self.muldiv(x, y, scale::into::<A, _>())
    }

    #[inline]
    fn div<const A: u8, B>(&self, x: semantic_fixed::Num<B>, y: semantic_fixed::Num<B>) -> Result<semantic_fixed::Num<B>> 
    where 
        B: int::Int {
        let scale: u128 = scale::into::<A, u128>();
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
                let (a, b) = wide_mul::calculate(x, y)?;
                if b >= z {
                    return Err(Error::Overflow);
                }
                if b == T::N0 {
                    return Ok(a / z);
                }
                Ok(fold::calculate(a, b, z)? / z)
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
        sign::to_negative(n)
    }
    
    #[inline]
    fn to_positive<T>(n: T) -> T 
    where 
        T: int::Int {
        sign::to_positive(n)
    }
}

impl<T> SignEngine for T 
where
    T: Engine 
{}


// --- --- ---

pub struct Q<const A: u8, B, C> 
where
    B: int::Int,
    C: Engine {
    v: B,
    engine: C
}


// --- --- ---

pub fn custom<const A: u8, B, C>(v: B, engine: C) -> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    Q { v, engine }
}


// --- --- ---

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
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
    C: Engine {
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
    C: Engine {
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
    C: Engine {
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
    C: Engine {
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
    C: Engine {
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
    C: Engine {
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let engine: C = self.engine;
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = engine.div::<A, _>(x, y)?;
        Ok(custom(out, engine))
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