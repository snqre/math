use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;

impl HasBrand for usize {

    fn brand(&self) -> Brand {
        Brand::USize
    }
}

impl HasSignIntrospection for usize {
    
    fn is_signed(&self) -> bool {
        false
    }
}