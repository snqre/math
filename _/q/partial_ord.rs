use super::*;

impl<T: _T> PartialOrd for Q<T> {
    fn gt(&self, other: &Self) -> bool {
        self._value > other._value
    }

    fn lt(&self, other: &Self) -> bool {
        self._value < other._value
    }
    
    fn ge(&self, other: &Self) -> bool {
        self._value >= other._value
    }

    fn le(&self, other: &Self) -> bool {
        self._value <= other._value
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}