use crate::math::i_branded::TrBranded;
use crate::math::i_branded::BrandedIBrand;
use crate::math::i_sign_introspection::SignIntrospectionI;

mod impl_branded_i {
    boiler::extend!();

    impl TrBranded for i8 {
        fn brand(&self) -> BrandedIBrand {
            BrandedIBrand::I8
        }
    }
}

mod impl_sign_introspection_i {
    boiler::extend!();

    impl SignIntrospectionI for i8 {
        fn is_signed(&self) -> bool {
            true
        }
    }
}