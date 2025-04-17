use num_traits::ToPrimitive;

use super::*;

impl<const A: u8, B: PrimInt> Mul for K<A, B> {
    type Output = KResult<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let decimals: u32 = Self::_only_safe_precision(A)?.into();
        let x: &Self = &self;
        let y: &Self = &rhs;
        let v_0: B = self._v;
        let v_1: B = rhs._v;
        debug_assert!(x.variant() == y.variant());
        match x.variant() {
            KVariant::I128 => {},
            KVariant::I64 => {},
            KVariant::I32 => {},
            KVariant::I16 => {},
            KVariant::I8 => {},
            KVariant::U128 => {
                let v_0: u128 = v_0.to_u128().unwrap();
                let v_1: u128 = v_0.to_u128().unwrap();
                let scale: u128 = Self::_scale_to_u128(decimals)?;
                let v_2: u128 = Self::_muldiv(v_0, v_1, scale)?;
                Ok(Self::_wu128(v_2)?)
            },
            KVariant::U64 => {
                let v_0: u64 = v_0.to_u64().unwrap();
                let v_1: u64 = v_1.to_u64().unwrap();
                let scale: u64 = Self::_scale_to_u64(decimals)?;
                let v_2: u64 = Self::_muldiv(v_0, v_1, scale)?;
                Ok(Self::_wrap_u64(v_2)?)
            },
        }
        

    }
}

if x.is_unsigned() {
    debug_assert!(y.is_unsigned());
    match x.variant() {
        KVariant::
        KVariant::U8 => {
            let v_0: u8 = v_0.to_u8().unwrap();
            let v_1: u8 = v_1.to_u8().unwrap();
            let scale: u8 = Self::_scale_to_u8(decimals)?;

        },
        
    }
}

if x.is_unsigned() && y.is_unsigned() {
    let v_0: u128 = v_0.to_u128().unwrap();
    let v_1: u128 = v_1.to_u128().unwrap();
    let scale: u128 = 10u128.pow(decimals);
    let v_2: u128 = Self::_muldiv(v_0, v_1, scale)?;
    let max: u128 = B::max_value().to_u128().unwrap();
    let v_2_safe: bool = v_2 <= max;
    if !v_2_safe { 
        let e: KError = KError::Overflow;
        Err(e)
    } else {
        let v_2: B = B::from(v_2).unwrap();
        Ok(k(v_2))
    }
} else if x.is_signed() && y.is_signed() {
    let v_0: i128 = v_0.to_i128().unwrap();
    let v_1: i128 = v_1.to_i128().unwrap();
    let scale: i128 = 10i128.pow(decimals);
    let v_2: i128 = Self::_muldiv(v_0, v_1, scale)?;
    let max: i128 = B::max_value().to_i128().unwrap();
    let min: i128 = B::min_value().to_i128().unwrap();
    let v_2_le_max: bool = v_2 <= max;
    let v_2_ge_min: bool = v_2 >= min;
    if !v_2_le_max {
        let e: KError = KError::Overflow;
        Err(e)
    } else if !v_2_ge_min {
        let e: KError = KError::Underflow;
        Err(e)
    } else {
        let v_2: B = B::from(v_2).unwrap();
        Ok(k(v_2))
    }
} else {
    panic!("")
}