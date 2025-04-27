#[cfg(feature = "ink")]
use ink::scale::Encode as IEncode;

pub trait MaybeEncodeT {}

#[cfg(feature = "ink")]
impl<T1> MaybeEncodeT for T1 
where 
    T1: IEncode {}

#[cfg(not(feature = "ink"))]
impl<T> MaybeEncodeTrait for T {}