use super::*;

impl<const A: u8, B: PrimInt + Branded> K<A, B> {
    pub fn max_representable(&self) -> KResult<B> {
        let decimals: u32 = Self::_only_safe_precision(A)?.into();
        match self._v.brand().as_str() {
            "u128" => {
                let scale: u128 = 10u128.checked_pow(decimals).ok_or(KError::Overflow)?;
                let cap: u128 = self.max_cap().to_u128().unwrap();
                let cap: B = B::from(cap).unwrap();
                Ok(cap)
            },
            "u64" => {
                let scale: u64 = 10u64.checked_pow(decimals).ok_or(KError::Overflow)?;
                let cap: u64 = self.max_cap().to_u64().unwrap();
                let cap: B = B::from(cap).unwrap();
                Ok(cap)
            },
            "u32" => {
                let scale: u32 = 10u32.checked_pow(decimals).ok_or(KError::Overflow)?;
                let cap: u32 = self.max_cap().to_u32().unwrap();
                let cap: B = B::from(cap).unwrap();
                Ok(cap)
            },
            "u16" => {
                let scale: u16 = Self::_scale_to_u16(decimals)?;
                let cap: u16 = self.max_cap().to_u16().unwrap();
                let cap: B = B::from(cap).unwrap();
                Ok(cap)
            },
            "u8" => {
                let scale: u8 = Self::_scale_to_u8(decimals)?;
                let cap: u8 = self.max_cap().to_u8().unwrap();
                let cap: B = B::from(cap).unwrap();
                Ok(cap)
            },
            "i128" => {
                
            },
            _ => {}
        }
    }

    pub fn min_representable(&self) -> KResult<B> {

    }
}