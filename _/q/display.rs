use crate::q::*;

impl<T: _T> Display for Q<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}..{}|", self._value, self._precision)
    }
}