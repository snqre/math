use super::*;

pub use super::Result;
pub use super::Error;

// --- --- ---

pub struct Point3D<const A: u8, B, C> 
where
    B: int::Int,
    C: q::Engine {
    pub x: q::Q<A, B, C>,
    pub y: q::Q<A, B, C>,
    pub z: q::Q<A, B, C>
}

// --- --- ---

impl<const A: u8, B, C> Point<A, B, C> for Point3D<A, B, C>
where
    B: int::Int,
    C: q::Engine {
    fn distance_between(&self, other: Self) -> Result<q::Q<A, B, C>> {
        let dx = other.x.clone() - self.x.clone();
        let dy = other.y.clone() - self.y.clone();
        let dz = other.z.clone() - self.z.clone();

        let dx2 = dx.clone() * dx;
        let dy2 = dy.clone() * dy;
        let dz2 = dz.clone() * dz;

        let sum = dx2 + dy2 + dz2;

        C::sqrt(sum).map_err(|e| Error::from_engine(e))
    }
}