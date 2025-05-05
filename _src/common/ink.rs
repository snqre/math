#[cfg(feature = "ink")]
use ink::scale;
#[cfg(feature = "ink")]
use ink::scale_info;
#[cfg(feature = "ink")]
use ink::storage::traits as storage;

pub trait Ink 
where
    Self: Decode,
    Self: Encode,
    Self: StorageLayout,
    Self: TypeInfo {}

impl<T> Ink for T 
where
    T: Decode,
    T: Encode,
    T: StorageLayout,
    T: TypeInfo {}

pub trait Decode {}

#[cfg(feature = "ink")]
impl<T> Decode for T 
where 
    T: scale::Decode {}

#[cfg(not(feature = "ink"))]
impl<T> Decode for T {}

pub trait Encode {}

#[cfg(feature = "ink")]
impl<T> Encode for T 
where 
    T: scale::Encode {}

#[cfg(not(feature = "ink"))]
impl<T> Encode for T {}

pub trait StorageLayout {}

#[cfg(feature = "ink")]
impl<T> StorageLayout for T 
where 
    T: storage::StorageLayout {}

#[cfg(not(feature = "ink"))]
impl<T> StorageLayout for T {}

pub trait TypeInfo {}

impl<T> TypeInfo for T 
where 
    T: scale_info::TypeInfo {}

#[cfg(not(feature = "ink"))]
impl<T> TypeInfo for T {}