use crate::extension::tr_branded::Branded;

impl Branded for i64 {
    fn brand(&self) -> &str {
        _BRAND
    }
}

const _BRAND: &str = "i64";