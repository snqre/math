use crate::common::util;
use crate::common::int;
use crate::math::q;
use crate::math::engine::default_engine;
use ::core::ops;
use ::core::fmt;
use ::thiserror as error;

use util::Util as _;

pub type Result<T> = ::core::result::Result<T, Error>;
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(error::Error)]
pub enum Error {
    #[error("")]
    Q(q::Error)
}
#[derive(Clone)]
pub struct Coordinate<const A: u8, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine {
    _x: q::Q<A, B, C>,
    _y: q::Q<A, B, C>
}

pub fn new<const A: u8, B, C>(
    x: q::Q<A, B, C>, 
    y: q::Q<A, B, C>) -> Coordinate<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine {
    Coordinate {
        _x: x,
        _y: y
    }
}

impl fmt::Debug for Coordinate {

}

impl fmt::Display for Coordinate {

}


//
//
// add 2 coordinates together to get a new coordinate which is
// moved more towards the added direction
impl<const A: u8, B, C> ops::Add for Coordinate<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine {
    type Output = Result<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        let coordinate_0: Self = self;
        let coordinate_1: Self = rhs;
        let x_0: q::Q<A, B, C> = coordinate_0._x;
        let y_0: q::Q<A, B, C> = coordinate_0._y;
        let x_1: q::Q<A, B, C> = coordinate_1._x;
        let y_1: q::Q<A, B, C> = coordinate_1._y;
        let x_2 = (x_0 + x_1).unwrap();
        let y_2 = (y_0 + y_1).unwrap();
        let result: Coordinate<A, B, C> = new(x_2, y_2);
        result.into_ok()
    }
}

impl ops::Sub for Coordinate {

}