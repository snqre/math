use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;
use crate::math::util_trait::ink::maybe_has_decode::MaybeHasDecode;
use crate::math::util_trait::ink::maybe_has_encode::MaybeHasEncode;
use crate::math::util_trait::ink::maybe_has_storage_layout::MaybeHasStorageLayout;
use crate::math::util_trait::ink::maybe_has_type_info::MaybeHasTypeInfo;
use ::num_traits::int::PrimInt as PrimInt;

pub trait IsInt:
    PrimInt
    + MaybeHasDecode
    + MaybeHasEncode
    + MaybeHasStorageLayout
    + MaybeHasTypeInfo
{}