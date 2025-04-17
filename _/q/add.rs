use super::*;

impl<T: _T> Add for Q<T> {
    type Output = QResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let precision_0: u32 = self._precision;
        let precision_1: u32 = rhs._precision;
        Self::_only_non_zero_precision(precision_0)?;
        Self::_only_non_zero_precision(precision_1)?;
        Self::_only_compatible_precision(precision_0, precision_1)?;
        let value_0: &T = &self._value;
        let value_1: &T = &rhs._value;
        let result: T = value_0
            .checked_add(value_1)
            .ok_or(QError::Overflow)?;
        let result: Self = Self {
            _value: result,
            _precision: precision_0,
        };
        Ok(result)
    }
}