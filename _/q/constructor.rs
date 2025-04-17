use super::*;

impl<T: _T> Q<T> {
    pub fn new(value: T, precision: u32) -> QResult<Self> {
        Self::_only_non_zero_precision(precision)?;
        let result: Self = Self {
            _value: value,
            _precision: precision,
        };
        Ok(result)
    }
}