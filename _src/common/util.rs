pub trait Util
where
    Self: Sized {

    fn into_ok<T>(self) -> Result<Self, T> {
        Ok(self)
    }

    fn into_err<T>(self) -> Result<T, Self> {
        Err(self)
    }

    fn into_some(self) -> Option<Self> {
        Some(self)
    }

    fn ignore(&self) {}
}

impl<T> Util for T {}