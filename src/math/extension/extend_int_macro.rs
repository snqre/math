#[macro_export]
macro_rules! extend_integer {
    ($int:ty, $brand:ident, $is_signed:expr) => {
        boiler::expose!(branded, sign_introspection);

        mod branded {
            boiler::extend!();

            impl BrandedTrait for $int {
                fn brand(&self) -> BrandedTraitBrand {
                    BrandedTraitBrand::$brand
                }
            }
        }

        mod sign_introspection {
            boiler::extend!();

            impl SignIntrospectionTrait for $int {
                fn is_signed(&self) -> bool {
                    $is_signed
                }
            }
        }
    };
}