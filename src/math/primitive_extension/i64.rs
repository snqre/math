use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;

impl HasBrand for i64 {

    fn brand(&self) -> Brand {
        Brand::I64
    }
}

impl HasSignIntrospection for i64 {
    
    fn is_signed(&self) -> bool {
        true
    }
}