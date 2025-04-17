use crate::extension::tr_branded::Branded;

impl Branded for u128 {
    fn brand(&self) -> &str {
        _BRAND
    }
}

const _BRAND: &str = "u128";