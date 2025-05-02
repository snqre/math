pub trait Encode {}

#[cfg(feature = "ink")]
impl<T> Encode for T 
    where
        T: ink::scale::Encode
{}

#[cfg(not(feature = "ink"))]
impl<T> Encode for T {}