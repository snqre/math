use super::*;

impl Branded for i8 {
    fn brand(&self) -> String {
        "i8".to_owned()
    }
}