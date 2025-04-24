use crate::math::i_branded::TrBranded;
use crate::math::i_sign_introspection::SignIntrospectionI;
use core::fmt::Debug;
use core::fmt::Display;
use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::ops::Rem;
use num_traits::int::PrimInt as PrimIntI;
use thiserror::Error;

pub trait QI<const A: u8, B: PrimIntI>:
      Sized
    + Debug
    + Display
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Add<Output = QIR<Self>>
    + Sub<Output = QIR<Self>>
    + Mul<Output = QIR<Self>>
    + Div<Output = QIR<Self>>
    + Rem<Output = QIR<Self>>
    + TryInto<i8, Error = QIE>
    + TryInto<i16, Error = QIE>
    + TryInto<i32, Error = QIE>
    + TryInto<i64, Error = QIE>
    + TryInto<i128, Error = QIE>
    + TryInto<isize, Error = QIE>
    + TryInto<u8, Error = QIE>
    + TryInto<u16, Error = QIE>
    + TryInto<u32, Error = QIE>
    + TryInto<u64, Error = QIE>
    + TryInto<u128, Error = QIE>
    + TryInto<usize, Error = QIE>
    + TryFrom<i8, Error = QIE>
    + TryFrom<i16, Error = QIE>
    + TryFrom<i32, Error = QIE>
    + TrBranded
    + SignIntrospectionI {
    fn max_value(&self) -> B;
    fn min_value(&self) -> B;
}

pub type QIR<T> = Result<T, QIE>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum QIE {
    #[error("")]
    Overflow,
    #[error("")]
    Underflow,
    #[error("")]
    DivisionByZero,
    #[error("")]
    RemByZero,
    #[error("")]
    PrecisionTooSmall,
    #[error("")]
    PrecisionTooLarge,
    #[error("")]
    IncompatiblePrecision,
    #[error("")]
    ConversionFailure,
}