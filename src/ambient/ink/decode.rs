pub trait Decode {}

#[cfg(feature = "ink")]
impl<T> Decode for T 
    where
        T: ink::scale::Decode
{}

#[cfg(not(feature = "ink"))]
impl<T> Decode for T {}