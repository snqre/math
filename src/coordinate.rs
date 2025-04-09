use crate::f_128::Q128I;
use crate::f_128::Q128E;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::PhantomData;
use thiserror::Error;

pub trait CoordinateI<T: Q128I<E>, E>:
    Sized
    + Debug
    + Display
    + Clone {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn distance(self, rhs: Self) -> Result<T, E>;
}


// --

pub type CoordinateR<T> = Result<T, CoordinateE>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum CoordinateE {
    #[error("Coordinate: ...")]
    F128E(#[from] Q128E),
}

#[derive(Clone)]
pub struct Coordinate<T: Q128I<E>, E> {
    __0: PhantomData<E>,
    x: T,
    y: T,
}

impl<T: Q128I<CoordinateE>> Coordinate<T, CoordinateE> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            __0: PhantomData,
            x,
            y,
        }
    }
}

impl<T: Q128I<CoordinateE>> CoordinateI<T, CoordinateE> for Coordinate<T, CoordinateE> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }
    
    fn distance(self, rhs: Self) -> CoordinateR<T> {
        let dfx: T = if self.x > rhs.x {
            (self.x - rhs.x)?
        } else {
            (rhs.x - self.x)?
        };
        let dfy: T = if self.y > rhs.y {
            (self.y - rhs.y)?
        } else {
            (rhs.y - self.y)?
        };
        let result: T = ((dfx * (dfy * dfy)?)?).sqrt()?;
        Ok(result)
    }
}

impl<T: Q128I<CoordinateE>> Debug for Coordinate<T, CoordinateE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate({},{})", self.x, self.y)
    }
}

impl<T: Q128I<CoordinateE>> Display for Coordinate<T, CoordinateE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate({},{})", self.x, self.y)
    }
}