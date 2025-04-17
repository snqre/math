use super::*;

impl Branded for i16 {
    fn brand(&self) -> String {
        "i16".to_owned()
    }
}