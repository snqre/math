pub trait MaybeHasEncode {}

#[cfg(feature = "ink")]
impl<T> MaybeHasEncode for T 
    where 
        T: ink::scale::Encode 
{}

#[cfg(not(feature = "ink"))]
impl<T> MaybeHasEncode for T {}