use crate::trait_::branded::Branded;

impl Branded for i64 {
    fn brand(&self) -> String {
        "i64".to_string()
    }
}