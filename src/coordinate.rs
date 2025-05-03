use crate::common::int;
use crate::math::q;
use crate::math::engine::default_engine;

pub struct Coordinate<const A: u8, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine {
    _x: q::Q<A, B, C>,
    _y: q::Q<A, B, C>
}

pub fn new<const A: u8, B>(
    x: q::Q<A, B, default_engine::DefaultEngine>, 
    y: q::Q<A, B, default_engine::DefaultEngine>) -> Coordinate<A, B, default_engine::DefaultEngine>
where
    B: int::Int,
    B: int::Introspection {
    Coordinate {
        _x: x,
        _y: y
    }
}

pub fn new_with_custom_engine<const A: u8, B, C>(
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