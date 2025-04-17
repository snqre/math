use super::*;

pub trait QUtilI<T0> {
    fn only_compatible_precision(precision_0: u8, precision_1: u8) -> Result<(), T0>;
    fn only_safe_precision(precision: u8) -> Result<(), T0>;
    fn scale_u128(precision: u8) -> Result<u128, T0>;
    fn scale_i128(precision: u8) -> Result<i128, T0>;
    fn muldiv_u128(x: u128, y: u128, z: u128) -> Result<u128, T0>;
    fn muldiv_i128(x: i128, y: i128, z: i128) -> Result<i128, T0>;
    fn to_highest_common_precision_qu128<T1: QU128I<T0>>(lhs: T1, rhs: T1) -> Result<(T1, T1), T0>;
    fn to_highest_common_precision_qi128<T1: QI128I<T0>>(lhs: T1, rhs: T1) -> Result<(T1, T1), T0>;
}

struct QUtil;

impl QUtilI<QE> for QUtil {
    
    fn only_compatible_precision(precision_0: u8, precision_1: u8) -> QR<()> {
        if precision_0 != precision_1 {
            let e: QE = QE::IncompatiblePrecision(precision_0, precision_1);
            return Err(e)
        }
        Ok(())
    }

    fn only_safe_precision(precision: u8) -> QR<()> {
        if precision < Q_MIN_PRECISION {
            let e: QE = QE::PrecisionTooSmall(precision, Q_MIN_PRECISION, Q_MAX_PRECISION);
            return Err(e)
        }
        if precision > Q_MAX_PRECISION {
            let e: QE = QE::PrecisionTooLarge(precision, Q_MIN_PRECISION, Q_MAX_PRECISION);
            return Err(e)
        }
        Ok(())
    }

    fn scale_u128(precision: u8) -> QR<u128> {
        Self::only_safe_precision(precision)?;
        let precision: u32 = precision.into();
        let result: u128 = 10u128
            .checked_pow(precision)
            .ok_or(QE::Overflow)
            .unwrap();
        Ok(result)
    }

    fn scale_i128(precision: u8) -> QR<i128> {
        let result: i128 = Self::scale_u128(precision)? as i128;
        Ok(result)
    }

    fn muldiv_u128(x: u128, y: u128, z: u128) -> QR<u128> {
        let result: u128 = x
            .checked_mul(y)
            .ok_or(QE::Overflow)?
            .checked_div(z)
            .ok_or(QE::DivisionByZero)?;
        Ok(result)
    }
}