use crate::extension::tr_branded::Branded;

impl Branded for i32 {
    fn brand(&self) -> &str {
        _BRAND
    }
}

const _BRAND: &str = "i32";