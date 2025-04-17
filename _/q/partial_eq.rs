use super::*;

impl<T: _T> PartialEq for Q<T> {
    fn eq(&self, other: &Self) -> bool {
        self._value == other._value
    }
}