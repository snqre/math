use crate::q::*;
use num_traits::Unsigned;

pub trait QUI<T0: Unsigned, T1>: Q<T0, T1> {}

#[derive(Clone)]
#[derive(Copy)]
pub struct QU<T0: Unsigned> {
    value: T0,
    precision: u8,
}

impl<T0: Unsigned> QU<T0> {

    pub fn new(value: T0, precision: u8) -> QR<Self> {
        
    }
}

impl<T0: Unsigned> QUI<T0, QE> for QU<T0> {}

impl<T0: Unsigned> Q<T0, QE> for QU<T0> {

    fn value(self) -> T0 {
        self.value
    }

    fn precision(self) -> u8 {
        self.precision
    }

    fn sqrt(self) -> Result<Self, QE> {
        
    }

    fn to_precision(self, new_precision: u8) -> Result<Self, QE> {
        
    }
}

impl<T0: Unsigned> Debug for QU<T0> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
    }
}

impl<T0: Unsigned> Display for QU<T0> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
    }
}

impl<T0> Add for QU<T0> {
    
    fn add(self, rhs: Self) -> Self::Output {
        
    }
}