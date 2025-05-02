use crate::ambient::ink::ink_compatible::InkCompatible;
use ::num_traits::int::PrimInt;
use ::num_traits::FromPrimitive;
use ::num_traits::ToPrimitive;

pub trait Int
    where
        Self: InkCompatible,
        Self: PrimInt,
        Self: FromPrimitive,
        Self: ToPrimitive 
{}

impl<T> Int for T 
    where
        T: InkCompatible,
        T: PrimInt,
        T: FromPrimitive,
        T: ToPrimitive
{}

pub mod introspection;