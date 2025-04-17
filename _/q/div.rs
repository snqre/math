use super::*;

impl<T: _T> Div for Q<T> {
    type Output = QResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let precision_0: u32 = self._precision;
        let precision_1: u32 = rhs._precision;
        Self::_only_non_zero_precision(precision_0)?;
        Self::_only_non_zero_precision(precision_1)?;
        Self::_only_compatible_precision(precision_0, precision_1)?;
        let value_0: T = self._value;
        let value_0: i128 = value_0
            .to_i128()
            .ok_or(QError::Unrepresentable)?;
        let value_1: T = rhs._value;
        let value_1: i128 = value_1
            .to_i128()
            .ok_or(QError::Unrepresentable)?;
        let scale: u128 = 10u128
            .checked_pow(precision_0)
            .ok_or(QError::Overflow)?;
        if !scale.is_power_of_two() {
            let scale: i128 = scale as i128;
            let result: i128 = value_0
                .checked_mul(scale)
                .ok_or(QError::Overflow)?
                .checked_div(value_1)
                .ok_or(QError::DivisionByZero)?;
            let result: T = T::from_i128(result).ok_or(QError::Unrepresentable)?;
            let result: Self = Self {
                _value: result,
                _precision: precision_0,
            };
            return Ok(result)
        }
        let scale_shift: u32 = scale.trailing_zeros();
        let result: i128 = value_0
            .checked_shl(scale_shift)
            .unwrap()
            .checked_div(value_1)
            .ok_or(QError::DivisionByZero)?;
        let result: T = T::from_i128(result).ok_or(QError::Unrepresentable)?;
        let result: Self = Self {
            _value: result,
            _precision: precision_0,
        };
        Ok(result)
    }
}