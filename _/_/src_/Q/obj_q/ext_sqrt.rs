boiler::extend!();

impl<const A: u8, B: PrimInt + Branded + SignIntrospection> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn sqrt(&self) -> Result<Self> {
        let v: B = self.v;
        if v < B::zero() {
            return Err(Error::Underflow)
        }
        let v: u128 = if self.is_signed() {
            let precision: u32 = A.into();
            let v: i128 = v
                .to_i128()
                .ok_or(Error::ConversionFailure)?
                .checked_shl(precision)
                .ok_or(Error::Overflow)?;
            if v < 0 {
                return Err(Error::Underflow)
            }
            let v: u128 = v.try_into().unwrap();
            Self::_int_sqrt_u128(v)?
        } else {
            let v: u128 = v
            .to_u128()
            .ok_or(Error::ConversionFailure)?;
            Self::_int_sqrt_u128(v)?
        };
        
         {
            false => {

            },
            _ => {

                match v < 0 {
                    true => , _ => (),
                }
            },
        }

        if v < B::zero() {
            return Err(Error::Underflow);
        }
        if self.is_signed() {
            let v: i128 = v.to_i128().unwrap();
            let v: i128 = v << A.into();
            
        }
        let v: u128 = v.into();
        let scale: u128 = 1u128 << A;

    }

    fn _int_sqrt_u128(v: u128) -> Result<u128> {
        if v < 2u128 {
            return Ok(v)
        }
        let mut l: u128 = 1u128;
        let mut r: u128 = v;
        while l <= r {
            let m: u128 = r
                .checked_sub(l)
                .ok_or(Error::Underflow)?
                .checked_add(l)
                .ok_or(Error::Overflow)?
                .checked_div(2u128)
                .unwrap();
            let m_sq: u128 = m.checked_mul(m).ok_or(Error::Overflow)?;
            if m_sq == v {
                return Ok(m)
            }
            if m_sq < v {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        Ok(r)
    }
}