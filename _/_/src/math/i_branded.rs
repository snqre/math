pub trait TrBranded {
    fn brand(&self) -> BrandedIBrand;
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum BrandedIBrand {
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