boiler::bundle!("src/common/tr_branded");

use num_traits::int::PrimInt;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Brand {
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

pub trait Branded: PrimInt {
    fn brand(&self) -> Brand;
}

boiler::expose!(
    for_i8,
    for_i16,
    for_i32,
    for_i64,
    for_i128,
    for_isize,
    for_u8,
    for_u16,
    for_u32,
    for_u64,
    for_u128,
    for_usize,
);