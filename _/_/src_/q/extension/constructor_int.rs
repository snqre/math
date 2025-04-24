boiler::extend!();

pub fn q_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Branded + SignIntrospection>(v: B) -> Result<Q::<C, D>> where 
    CheckPrecision<A>: IsPrecision,
    CheckPrecision<C>: IsPrecision, {
    Q::new_from_int::<A, B>(v)
}

impl<const A: u8, B: PrimInt + Branded + SignIntrospection> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> Result<Self> where CheckPrecision<C>: IsPrecision {
        let v: B = B::from(v).ok_or(Error::ConversionFailure)?;
        let v: Q<C, B> = q(v);
        let v: Q<A, B> = v.cast()?;
        Ok(v)
    }
}