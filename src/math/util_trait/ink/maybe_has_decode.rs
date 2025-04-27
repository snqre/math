pub trait MaybeHasDecode {}

#[cfg(feature = "ink")]
impl<T_: ink::scale::Decode> MaybeHasDecode for T_ {}

#[cfg(not(feature = "ink"))]
impl<T_> MaybeHasDecode for T_ {}