use ::{
    core::{
        ops::{ Sub }
    }
};

use super::{
    Int, 
    Engine, 
    Q, 
    Result
};

impl<const A: u8, B: Int, C: Engine> Sub for Q<A, B, C> {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = self.engine.sub(x, y)?;
        let out: Self = Self {
            v: out,
            engine: self.engine
        };
        Ok(out)
    }
}