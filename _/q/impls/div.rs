use ::{
    core::{
        ops::{ Div }
    }
};

use super::{
    Int, 
    Engine, 
    Q, 
    Result
};

impl<const A: u8, B: Int, C: Engine> Div for Q<A, B, C> {
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let x: B = self.v;
        let y: B = rhs.v;
        let out: B = self.engine.div::<A, _>(x, y)?;
        let out: Self = Self {
            v: out,
            engine: self.engine
        };
        Ok(out)
    }
}