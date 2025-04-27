use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;

impl HasBrand for i128 {

    fn brand(&self) -> Brand {
        Brand::I128
    }
}

impl HasSignIntrospection for i128 {
    
    fn is_signed(&self) -> bool {
        true
    }
}