pub trait StorageLayout {}

#[cfg(feature = "ink")]
impl<T> StorageLayout for T 
    where
        T: ink::storage::traits::StorageLayout
{}

#[cfg(not(feature = "ink"))]
impl<T> StorageLayout for T {}