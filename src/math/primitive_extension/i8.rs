use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;

impl HasBrand for i8 {

    fn brand(&self) -> Brand {
        Brand::I8
    }
}

impl HasSignIntrospection for i8 {
    
    fn is_signed(&self) -> bool {
        true
    }
}