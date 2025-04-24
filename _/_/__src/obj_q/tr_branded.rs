boiler::extend!();

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub(super) enum _Brand {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

pub(super) trait _Branded: PrimInt {
    fn brand(&self) -> _Brand;
}