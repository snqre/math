use crate::extension::tr_branded::Branded;

impl Branded for u8 {
    fn brand(&self) -> &str {
        _BRAND
    }
}

const _BRAND: &str = "u8";