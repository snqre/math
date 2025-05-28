use super::*;

pub use super::Result;
pub use super::Error;

pub struct Point4D<const A: u8, B, C> 
where
    B: int::Int,
    C: q::Engine {
    timestamp: B,
    x: q::Q<A, B, C>,
    y: q::Q<A, B, C>,
    z: q::Q<A, B, C>
}

impl<const A: u8, B, C> Point<A, B, C> for Point4D<A, B, C>
where
    B: int::Int,
    C: q::Engine {
    fn distance_between(&self, other: Self) -> Result<q::Q<A, B, C>> {
        // Convert timestamps to Q<A, B, C>
        let t1 = q::Q::<A, B, C>::from_int(self.timestamp.clone())?;
        let t2 = q::Q::<A, B, C>::from_int(other.timestamp.clone())?;

        let dt = t2 - t1;
        let dx = other.x.clone() - self.x.clone();
        let dy = other.y.clone() - self.y.clone();
        let dz = other.z.clone() - self.z.clone();

        let dt2 = dt.clone() * dt;
        let dx2 = dx.clone() * dx;
        let dy2 = dy.clone() * dy;
        let dz2 = dz.clone() * dz;

        let sum = dt2 + dx2 + dy2 + dz2;

        C::sqrt(sum).map_err(Error::from_engine)
    }
}

type T = Point4D<2, u128, q::DefaultEngine>;