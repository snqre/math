boiler::expose!(impl_branded, impl_sign_introspection);

use crate::math::branded_trait::BrandedTrait;
use crate::math::branded_trait::BrandedTraitBrand;
use crate::math::sign_introspection_trait::SignIntrospectionTrait;

mod impl_branded {
    boiler::extend!();

    impl BrandedTrait for i16 {
        fn brand(&self) -> BrandedTraitBrand {
            BrandedTraitBrand::I16
        }
    }
}

mod impl_sign_introspection {
    boiler::extend!();

    impl SignIntrospectionTrait for i16 {
        fn is_signed(&self) -> bool {
            true
        }
    }
}