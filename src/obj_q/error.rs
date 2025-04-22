boiler::extend!();

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Error)]
pub enum Error {
    #[error("")]
    InsufficientRangeToSupportPrecision,
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