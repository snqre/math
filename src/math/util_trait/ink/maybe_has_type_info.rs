pub trait MaybeHasTypeInfo {}

#[cfg(feature = "ink")]
impl<T> MaybeHasTypeInfo for T 
    where 
        T: ink::scale_info::TypeInfo 
{}

#[cfg(not(feature = "ink"))]
impl<T> MaybeHasTypeInfo for T {}