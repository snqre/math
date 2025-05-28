use super::*;

pub use super::Result;
pub use super::Error;

// --- --- ---

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Point2D<const A: u8, B, C>
where
    B: int::Int,
    C: q::Engine {
    pub x: q::Q<A, B, C>,
    pub y: q::Q<A, B, C>
}

// --- --- ---

impl<const A: u8, B, C> Point2D<A, B, C>
where
    B: int::Int,
    C: q::Engine {
    
}

// --- --- ---

impl<const A: u8, B, C> Point<A, B, C> for Point2D<A, B, C>
where
    B: int::Int,
    C: q::Engine {
    fn distance_between(&self, other: Self) -> Result<q::Q<A, B, C>> {
        (|| -> q::Result<q::Q<A, B, C>> {
            let dx: q::Q<A, B, C> = (self.x - other.x)?;
            let dx: q::Q<A, B, C> = (dx * dx)?;
            let dy: q::Q<A, B, C> = (self.y - other.y)?;
            let dy: q::Q<A, B, C> = (dy * dy)?;
            let sum: q::Q<A, B, C> = (dx + dy)?;
            let sum: q::Q<A, B, C> = sum.sqrt();
            Ok(sum)
        })().map_err(Error::Q)
    }
}