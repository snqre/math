pub use num_traits::PrimInt as Int;

pub trait Introspection where Self: Int {

    fn is_unsigned(&self) -> bool {
        !self.is_signed()
    }

    fn is_signed(&self) -> bool {
        Self::min_value() < Self::zero()
    }

    fn to_int<T>(&self) -> Option<T> where T: Int {
        if T::zero().is_signed() {
            let max: i128 = T::max_value().to_i128().unwrap();
            let min: i128 = T::min_value().to_i128().unwrap();
            let value: i128 = self.to_i128().unwrap();
            if value > max {
                return None;
            }
            if value < min {
                return None;
            }
            let value: T = T::from(value)?;
            return Some(value)
        }
        let max: u128 = T::max_value().to_u128().unwrap();
        let min: u128 = T::min_value().to_u128().unwrap();
        let value: u128 = self.to_u128().unwrap();
        if value > max {
            return None;
        }
        if value < min {
            return None;
        }
        let value: T = T::from(value)?;
        Some(value)
    }
}

impl<T> Introspection for T where T: Int {}