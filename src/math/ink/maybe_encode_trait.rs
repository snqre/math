#[cfg(feature = "ink")]
use ink::scale::Encode as EncodeTrait;

pub trait MaybeEncodeTrait {}

#[cfg(feature = "ink")]
impl<T> MaybeEncodeTrait for T 
where 
    T: EncodeTrait {}

#[cfg(not(feature = "ink"))]
impl<T> MaybeEncodeTrait for T {}