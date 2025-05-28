use ::core::marker;

use super::*;

mod add_impl;
mod arithmetic_engine;
mod div_impl;
mod engine;
mod fold;
mod mul_impl;
mod muldiv_engine;
mod ord_impl;
mod partial_eq_impl;
mod pi;
mod scale;
mod sign_engine;
mod sqrt_impl;
mod sub_impl;
mod trig_conversion_engine;
mod trig_conversion_impl;
mod trig_engine;
mod trig_hyperbolic_engine;
mod trig_hyperbolic_impl;
mod trig_impl;
mod trig_reciprocal_engine;
mod trig_reciprocal_impl;
mod trig;
mod var_generic;
mod var;
mod wide_mul;

pub use arithmetic_engine::ArithmeticEngine;
pub use engine::Engine;
pub use muldiv_engine::MuldivEngine;
pub use sign_engine::SignEngine;
pub use trig_conversion_engine::TrigConversionEngine;
pub use trig_engine::TrigEngine;
pub use trig_hyperbolic_engine::TrigHyperbolicEngine;
pub use trig_reciprocal_engine::TrigReciprocalEngine;
pub use var_generic::*;
pub use var::*;

// --- --- ---

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
pub struct DefaultEngine;

impl Engine for DefaultEngine {}

// --- --- ---

pub type Default<const A: u8, B> = Q<A, B, DefaultEngine>;

// --- --- ---

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Error {
    Overflow,
    Underflow,
    DivisionByZero
}

// --- --- ---

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B, C> 
where
    B: int::Int,
    C: Engine {
    __0: marker::PhantomData<C>,
    v: B
}

// --- --- ---

#[inline]
pub fn new<const A: u8, B, C>(v: B) -> Q<A, B, C> 
where
    B: int::Int,
    C: Engine {
    Q::new(v)
}

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    #[inline]
    pub fn new(v: B) -> Self {
        Self { v, __0: marker::PhantomData }
    }
}

// --- --- ---

impl<const A: u8, B, C> From<B> for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    fn from(value: B) -> Self {
        new(value)
    }
}