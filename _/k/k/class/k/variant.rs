use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum KVariant {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
}

impl<const A: u8, B: PrimInt + Branded> K<A, B> {
    pub fn variant(&self) -> KVariant {
        match self._v.brand().as_str() {
            "u8" => KVariant::U8,
            "u16" => KVariant::U16,
            "u32" => KVariant::U32,
            "u64" => KVariant::U64,
            "u128" => KVariant::U128,
            "i8" => KVariant::I8,
            "i16" => KVariant::I16,
            "i32" => KVariant::I32,
            "i64" => KVariant::I64,
            "i128" => KVariant::I128,
            _ => panic!("Non primitive numerical value."),
        }
    }

    pub fn is_signed(&self) -> bool {
        match self.variant() {
            KVariant::I128 => true,
            KVariant::I64 => true,
            KVariant::I32 => true,
            KVariant::I16 => true,
            KVariant::I8 => true,
            _ => false,
        }
    }

    pub fn is_unsigned(&self) -> bool {
        !self.is_signed()
    }
}