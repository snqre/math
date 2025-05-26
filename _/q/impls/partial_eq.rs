use super::{ Int, Engine, Q };

impl<const A: u8, B: Int, C: Engine> PartialEq for Q<A, B, C> {
    fn eq(&self, other: &Self) -> bool {
        let x: B = self.v;
        let y: B = other.v;
        x == y
    }
}