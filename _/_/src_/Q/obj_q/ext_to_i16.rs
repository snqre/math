boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn to_i16(&self) -> Result<i16> {
        self.v.to_i16().ok_or(Error::ConversionFailure)
    }
}