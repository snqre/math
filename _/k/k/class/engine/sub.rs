use super::*;

impl Engine {
    pub fn sub<T: PrimInt>(&self, x: &T, y: &T) -> Result<T> {
        x.checked_sub(y).ok_or(Error::Underflow)
    }
}