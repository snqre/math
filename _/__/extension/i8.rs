use crate::trait_::branded::Branded;

impl Branded for i8 {
    fn brand(&self) -> String {
        "i8".to_owned()
    }
}