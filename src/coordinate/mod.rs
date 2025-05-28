use crate::common::int;
use crate::common::q;

pub mod point_2d;
pub mod point_3d;
pub mod point_4d;
pub mod point_n;

// --- --- ---

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
pub enum Error {
    Q(q::Error)
}

// --- --- ---

pub trait Point<const A: usize, const B: u8, C, D>
where
    C: int::Int,
    D: q::Engine {
    fn dimension() -> [q::Q<B, C, D>; A];
    fn distance_between(&self, other: Self) -> Result<q::Q<B, C, D>>;
}