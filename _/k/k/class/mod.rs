use super::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum Error {
    #[error("")]
    Overflow,
    #[error("")]
    Underflow,
    #[error("")]
    DivisionByZero,
    #[error("")]
    PrecisionTooSmall,
    #[error("")]
    PrecisionTooLarge,
    #[error("")]
    IncompatiblePrecision,
}

pub struct K<const A: u8, B: PrimInt> {
    _v: B,
}

pub fn k<const A: u8, B: PrimInt>(v: B) -> K::<A, B> {
    K::new(v)
}

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn new(v: B) -> Self {
        Self {
            _v: v,
        }
    }
}

impl<const A: u8, B: PrimInt> K<A, B> {
    pub fn cast<const C: u8>(&self) -> Result<K<C, B>> {
        let old_decimals: u32 = 
    }
}

impl<const A: u8, B: PrimInt> Add for K<A, B> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = engine::Engine::new_default().add(v_0, v_1)?;
        Ok(k(v_2))
    }
}

impl<const A: u8, B: PrimInt> Sub for K<A, B> {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: &B = &x._v;
        let v_1: &B = &y._v;
        let v_2: B = engine::Engine::new_default().sub(v_0, v_1)?;
        Ok(k(v_2))
    }
}

impl<const A: u8, B: PrimInt> PartialOrd for K<A, B> {
    fn gt(&self, other: &Self) -> bool {
        self._v > other._v
    }

    fn lt(&self, other: &Self) -> bool {
        self._v < other._v
    }

    fn ge(&self, other: &Self) -> bool {
        self._v >= other._v
    }

    fn le(&self, other: &Self) -> bool {
        self._v <= other._v
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const A: u8, B: PrimInt> Ord for K<A, B> {
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else if self < other {
            other
        } else {
            self
        }
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else if self > other {
            other
        } else {
            self
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self > other {
            Ordering::Greater
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl<const A: u8, B: PrimInt> PartialEq for K<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self._v == other._v
    }
}

impl<const A: u8, B: PrimInt> Eq for K<A, B> {}