use crate::num::engine::default_engine;
use crate::num::int;
use crate::num::int_introspection;
use crate::num::precision;
use crate::num::q;

use ::core::fmt;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::thiserror::Error)]
pub enum Error {
    #[error("")]
    Q(#[from] q::Error)
}

#[derive(Clone)]
pub struct Point2D<const A: u8, B, C> 
where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    C: q::Engine, 
    precision::Precision<A>: precision::Compatible {
    _x: q::Q<A, B, C>,
    _y: q::Q<A, B, C>
}

pub fn new<const A: u8, B, C>(
    x: &q::Q<A, B, C>,
    y: &q::Q<A, B, C>
) -> Point2D<A, B, C> 
where 
    B: int::Int,
    B: int_introspection::IntIntrospection,
    C: q::Engine, 
    precision::Precision<A>: precision::Compatible {
    Point2D {
        _x: x.clone(),
        _y: y.clone()
    }
}

pub fn default<const A: u8, B>() -> Point2D<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    precision::Precision<A>: precision::Compatible {
    let zero: &B = &B::zero();
    let x: q::Q<A, B, default_engine::DefaultEngine> = q::new(zero);
    let y: q::Q<A, B, default_engine::DefaultEngine> = q::new(zero);
    Point2D {
        _x: x,
        _y: y
    }
}