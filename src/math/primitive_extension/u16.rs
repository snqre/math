use crate::math::util_trait::has_brand::HasBrand;
use crate::math::util_trait::has_brand::Brand;
use crate::math::util_trait::has_sign_introspection::HasSignIntrospection;

impl HasBrand for u16 {

    fn brand(&self) -> Brand {
        Brand::U16
    }
}

impl HasSignIntrospection for u16 {
    
    fn is_signed(&self) -> bool {
        false
    }
}