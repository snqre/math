pub trait MaybeHasDecode {}

#[cfg(feature = "ink")]
impl<T> MaybeHasDecode for T
    where
        T: ink::scale::Decode
{}

#[cfg(not(feature = "ink"))]
impl<T> MaybeHasDecode for T {}