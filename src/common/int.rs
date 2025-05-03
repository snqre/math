use crate::common::ink::Ink;
use crate::common::util::Util;
use ::num_traits::FromPrimitive;
use ::num_traits::ToPrimitive;
use ::num_traits::int::PrimInt;

pub trait Int
where
    Self: Ink,
    Self: PrimInt,
    Self: FromPrimitive,
    Self: ToPrimitive {}

impl<T> Int for T 
where
    T: Ink,
    T: PrimInt,
    T: FromPrimitive,
    T: ToPrimitive {}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[repr(u8)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
}

pub trait Introspection
where
    Self: Int {

    fn r#type(&self) -> Type;

    fn is_signed(&self) -> bool {
        match self.r#type() {
            Type::U8 => false,
            Type::U16 => false,
            Type::U32 => false,
            Type::U64 => false,
            Type::U128 => false,
            Type::I8 => true,
            Type::I16 => true,
            Type::I32 => true,
            Type::I64 => true,
            Type::I128 => true,
        }
    }

    fn is_unsigned(&self) -> bool {
        !self.is_signed()
    }

    fn as_signed<T>(&self) -> Option<T>
    where
        T: Int,
    {
        if self.is_signed() {
            let r: T = T::from(*self).unwrap();
            return r.into_some();
        }
        None
    }

    fn as_unsigned<T>(&self) -> Option<T>
    where
        T: Int,
    {
        if self.is_signed() {
            return None;
        }
        let r: T = T::from(*self).unwrap();
        r.into_some()
    }

    fn to_int<T>(&self) -> Option<T>
    where
        T: Int,
    {
        if Self::zero().is_signed() {
            let x: i128 = self.to_i128()?;
            let max: i128 = Self::max_value().to_i128()?;
            let min: i128 = Self::min_value().to_i128()?;
            if x > max || x < min {
                return None;
            }
            let x: T = T::from_i128(x)?;
            return x.into_some();
        }
        let x: u128 = self.to_u128()?;
        let max: u128 = Self::max_value().to_u128()?;
        let min: u128 = Self::min_value().to_u128()?;
        if x > max || x < min {
            return None;
        }
        let x: T = T::from_u128(x)?;
        x.into_some()
    }
}

macro_rules! impls {
    ($(($size:ty, $type:expr)),*) => {
        $(
            impl Introspection for $size 
            where
                Self: Int {

                fn r#type(&self) -> Type {
                    $type
                }
            }
        )*
    };
} impls!(
    (i8, Type::I8),
    (i16, Type::I16),
    (i32, Type::I32),
    (i64, Type::I64),
    (i128, Type::I128),
    (u8, Type::U8),
    (u16, Type::U16),
    (u32, Type::U32),
    (u64, Type::U64),
    (u128, Type::U128)
);