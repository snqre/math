use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;

impl HasBrand for i16 {

    fn brand(&self) -> Brand {
        Brand::I16
    }
}

impl HasSignIntrospection for i16 {
    
    fn is_signed(&self) -> bool {
        true
    }
}