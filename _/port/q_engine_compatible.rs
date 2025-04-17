use thiserror::Error;

pub trait QEngineCompatible<T0> {
    fn only_compatible_precision(precision_0: u8, precision_1: u8) -> Result<(), T0>;
    fn only_safe_precision(precision: u8) -> Result<(), T0>;
    fn scale_u128(precision: u8) -> Result<u128, T0>;
    fn scale_i128(precision: u8) -> Result<i128, T0>;
    fn muldiv_u128(x: u128, y: u128, z: u128) -> Result<u128, T0>;
    fn muldiv_i128(x: i128, y: i128, z: i128) -> Result<i128, T0>;
    fn to_highest_common_precision_qu128<T1: QU128I<T0>>(lhs: T1, rhs: T1) -> Result<(T1, T1), T0>;
    fn to_highest_common_precision_qi128<T1: QI128I<T0>>(lhs: T1, rhs: T1) -> Result<(T1, T1), T0>;
}