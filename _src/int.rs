



pub trait IntI: ink::Ink
    + num::PrimInt
    + num::FromPrimitive
    + num::ToPrimitive {}

pub trait IntIntrospectionI: IntI {

    fn variant(&self) -> IntIntrospectionIVariant;

    fn is_unsigned(&self) -> bool {
        !self.is_signed()
    }

    fn is_signed(&self) -> bool {
        match self.variant() {
            IntIntrospectionIVariant::U8 => false,
            IntIntrospectionIVariant::U16 => false,
            IntIntrospectionIVariant::U32 => false,
            IntIntrospectionIVariant::U64 => false,
            IntIntrospectionIVariant::U128 => false,
            IntIntrospectionIVariant::I8 => true,
            IntIntrospectionIVariant::I16 => true,
            IntIntrospectionIVariant::I32 => true,
            IntIntrospectionIVariant::I64 => true,
            IntIntrospectionIVariant::I128 => true
        }
    }

    fn to_int<T: IntI>(&self) -> Option<T> {
        if Self::zero().is_signed() {
            let max: i128 = Self::max_value().to_i128()?;
            let min: i128 = Self::min_value().to_i128()?;
            let value: i128 = self.to_i128()?;
            if value > max || value < min {
                return None
            }
            let value: T = T::from_i128(value)?;
            return Some(value)
        }

    }
}

pub enum IntIntrospectionIVariant {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128
}

impl IntIntrospectionI for u8 {

    fn variant(&self) -> IntIntrospectionIVariant {
        IntrospectionIVariant::U8
    }
}

impl IntIntrospectionI for u16 {

    fn variant(&self) -> IntIntrospectionIVariant {
        IntIntrospectionIVariant::U16
    }
}

