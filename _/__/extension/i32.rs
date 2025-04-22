use crate::trait_::branded::Branded;

impl Branded for i32 {
    fn brand(&self) -> String {
        "i32".to_owned()
    }
}