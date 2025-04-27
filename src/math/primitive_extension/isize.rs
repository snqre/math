use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;

impl HasBrand for isize {

    fn brand(&self) -> Brand {
        Brand::ISize
    }
}

impl HasSignIntrospection for isize {
    
    fn is_signed(&self) -> bool {
        true
    }
}