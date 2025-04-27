pub trait MaybeTypeInfoTrait {}

#[cfg(feature = "ink")]
impl<T> MaybeTypeInfoTrait for T where T: scale_info::TypeInfo {}

#[cfg(not(feature = "ink"))]
impl<T> MaybeTypeInfo for T {}