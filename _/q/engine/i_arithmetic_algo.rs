use super::{
    Int,
    Fixed,
    IMuldivAlgo,
    IScaleAlgo
};

pub trait IArithmeticAlgo
where
    Self: IMuldivAlgo,
    Self: scale::Engine {

}

impl<T> Engine for T
where
    T: super::Handler 
{}