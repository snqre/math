use crate::interface::branded::Branded;

impl Branded for i64 {
    fn brand(&self) -> String {
        "i64".to_owned()
    }
}