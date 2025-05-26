use super::*;

impl<const A: u8, B: Int, C: Engine> ::core::ops::Add for Q<A, B, C> {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = self.engine.add(x, y)?;
        let out: Self = Self {
            v: out,
            engine: self.engine
        };
        Ok(out)
    }
}