pub trait MaybeHasStorageLayout {}

#[cfg(feature = "ink")]
impl<T> MaybeHasStorageLayout for T 
    where 
        T: ink::storage::traits::StorageLayout
{}

#[cfg(not(feature = "ink"))]
impl<T> MaybeHasStorageLayout for T {}