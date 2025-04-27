#[cfg(feature = "ink")]
use ink::scale::Decode as DecodeT;

pub trait MaybeDecodeT {}

#[cfg(feature = "ink")]
impl<T1> MaybeDecodeT for T1 
    where 
        T1: DecodeT {}

#[cfg(not(feature = "ink"))]
impl<T1> MaybeDecodeT for T1 {}