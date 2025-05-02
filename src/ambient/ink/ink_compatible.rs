use crate::ambient::ink::type_info::TypeInfo;
use crate::ambient::ink::decode::Decode;
use crate::ambient::ink::encode::Encode;
use crate::ambient::ink::storage_layout::StorageLayout;

pub trait InkCompatible
    where
        Self: TypeInfo,
        Self: Decode,
        Self: Encode,
        Self: StorageLayout
{}

impl<T> InkCompatible for T
    where
        T: TypeInfo,
        T: Decode,
        T: Encode,
        T: StorageLayout
{}