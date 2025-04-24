#[cfg(feature = "ink")]
use ink::scale::Decode as DecodeTrait;

pub trait MaybeDecodeTrait {}

#[cfg(feature = "ink")]
impl<T> MaybeDecodeTrait for T 
where 
    T: DecodeTrait {}

#[cfg(not(feature = "ink"))]
impl<T> MaybeDecodeTrait for T {}