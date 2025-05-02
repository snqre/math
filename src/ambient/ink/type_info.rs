pub trait TypeInfo {}

#[cfg(feature = "ink")]
impl<T> TypeInfo for T 
    where
        T: ink::scale_info::TypeInfo
{}

#[cfg(not(feature = "ink"))]
impl<T> TypeInfo for T {}