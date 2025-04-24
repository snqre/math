boiler::expose!(impl_branded, impl_sign_introspection);

use crate::math::branded_trait::BrandedTrait;
use crate::math::branded_trait::BrandedTraitBrand;
use crate::math::sign_introspection_trait::SignIntrospectionTrait;

mod impl_branded {
    boiler::extend!();

    impl BrandedTrait for i8 {
        fn brand(&self) -> BrandedTraitBrand {
            BrandedTraitBrand::I8
        }
    }
}

mod impl_sign_introspection {
    boiler::extend!();

    impl SignIntrospectionTrait for i8 {
        fn is_signed(&self) -> bool {
            true
        }
    }
}