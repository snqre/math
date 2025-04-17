use crate::q::*;

impl<T: _T> Mul for Q<T> {
    type Output = QResult<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let precision_0: u32 = self._precision;
        let precision_1: u32 = rhs._precision;
        Self::_only_non_zero_precision(precision_0)?;
        Self::_only_non_zero_precision(precision_1)?;
        Self::_only_compatible_precision(precision_0, precision_1)?;
        if self._value < T::zero() || rhs._value < T::zero() {
            let value_0: T = self._value;
            let value_0: i128 = value_0
                .to_i128()
                .ok_or(QError::Unrepresentable)?;
            let value_1: T = rhs._value;
        } else {

        }
        let value_0: T = self._value;
        let value_0: i128 = value_0
            .to_usize()
        let value_1: T = rhs._value;
        let scale: T = 


        let scale: i128 = 10i128
            .checked_pow(precision_0)
            .ok_or(QError::Overflow)?;

        
        value_0
            .checked_mul(value_1)
            .ok_or(QError::Overflow)?
            .checked_div(scale)
            .ok_or(QError::DivisionByZero)?;


        only_safe_precision(self.precision)?; 
        only_safe_precision(rhs.precision)?;
        only_compatible_precision(self.precision, rhs.precision)?;

        let decimals: u32 = self.precision.into();

        // Calculate scale based on precision
        let scale: u128 = 10u128.checked_pow(decimals).ok_or(Q128E::Overflow)?;

        // Perform the multiplication and scale
        let result: u128 = self.value.checked_mul(rhs.value).ok_or(Q128E::Overflow)?;
        let scaled_result: u128 = result.checked_div(scale).ok_or(Q128E::Overflow)?;

        let result: Self = Self {
            value: scaled_result,
            precision: self.precision,
        };
        
        Ok(result)
    }
}