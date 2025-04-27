pub trait MaybeStorageLayoutTrait {}

#[cfg(feature = "ink")]
impl<T> MaybeStorageLayoutTrait for T where T: ink::storage::traits::StorageLayout {}

#[cfg(not(feature = "ink"))]
impl<T> MaybeStorageLayoutTrait for T {}