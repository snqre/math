use crate::interface::branded::Branded;

impl Branded for i32 {
    fn brand(&self) -> String {
        "i32".to_owned()
    }
}