use super::*;

impl<const A: u8, B: PrimInt> Display for K<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(v) = self.to_f32() {
            
        } else {
            let v: u128 = self.to_u128();
            write!(f, "{}", v)
        }
    }
}