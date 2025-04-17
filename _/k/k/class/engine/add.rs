use super::*;

impl Engine {
    pub fn add<T: PrimInt>(&self, x: &T, y: &T) -> Result<T> {
        x.checked_add(y).ok_or(Error::Overflow)
    }
}