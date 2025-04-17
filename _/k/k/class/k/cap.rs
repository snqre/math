use super::*;

impl<const A: u8, B: PrimInt + Branded> K<A, B> {
    pub fn max_cap(&self) -> B {
        use std::i128::MAX as MAX_I128;
        use std::i64::MAX as MAX_I64;
        use std::i32::MAX as MAX_I32;
        use std::i16::MAX as MAX_I16;
        use std::i8::MAX as MAX_I8;
        use std::u128::MAX as MAX_U128;
        use std::u64::MAX as MAX_U64;
        use std::u32::MAX as MAX_U32;
        use std::u16::MAX as MAX_U16;
        use std::u8::MAX as MAX_U8;
        match self._v.brand().as_str() {
            "i128" => B::from(MAX_I128).unwrap(),
            "i64" => B::from(MAX_I64).unwrap(),
            "i32" => B::from(MAX_I32).unwrap(),
            "i16" => B::from(MAX_I16).unwrap(),
            "i8" => B::from(MAX_I8).unwrap(),
            "u128" => B::from(MAX_U128).unwrap(),
            "u64" => B::from(MAX_U64).unwrap(),
            "u32" => B::from(MAX_U32).unwrap(),
            "u16" => B::from(MAX_U16).unwrap(),
            "u8" => B::from(MAX_U8).unwrap(),
            _ => panic!(""),
        }
    }

    pub fn min_cap(&self) -> B {
        use std::i128::MIN as MIN_I128;
        use std::i64::MIN as MIN_I64;
        use std::i32::MIN as MIN_I32;
        use std::i16::MIN as MIN_I16;
        use std::i8::MIN as MIN_I8;
        use std::u128::MIN as MIN_U128;
        use std::u64::MIN as MIN_U64;
        use std::u32::MIN as MIN_U32;
        use std::u16::MIN as MIN_U16;
        use std::u8::MIN as MIN_U8;
        match self._v.brand().as_str() {
            "i128" => B::from(MIN_I128).unwrap(),
            "i64" => B::from(MIN_I64).unwrap(),
            "i32" => B::from(MIN_I32).unwrap(),
            "i16" => B::from(MIN_I16).unwrap(),
            "i8" => B::from(MIN_I8).unwrap(),
            "u128" => B::from(MIN_U128).unwrap(),
            "u64" => B::from(MIN_U64).unwrap(),
            "u32" => B::from(MIN_U32).unwrap(),
            "u16" => B::from(MIN_U16).unwrap(),
            "u8" => B::from(MIN_U8).unwrap(),
            _ => panic!(""),
        }
    }
}