use crate::extension::tr_branded::Branded;

impl Branded for u64 {
    fn brand(&self) -> &str {
        _BRAND
    }
}

const _BRAND: &str = "u64";