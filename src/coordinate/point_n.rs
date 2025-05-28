use super::*;

pub struct PointN<const A: usize, const B: u8, C, D>
where
    C: int::Int,
    D: q::Engine {
    pub dimension: [q::Q<B, C, D>; A]
}

impl<const A: usize, const B: u8, C, D> Point<B, C, D> for PointN<A, B, C, D>
where
    C: int::Int,
    D: q::Engine {
    fn distance_between(&self, other: Self) -> Result<q::Q<B, C, D>> {
        let mut sum = q::zero();
        for (a, b) in self.dimension.iter().zip(other.dimension.iter()) {
            let d = (b.clone() - a.clone()).unwrap();
            sum = ((sum + d).unwrap() * d).unwrap();
        }
        Ok(sum.sqrt())
    }
}