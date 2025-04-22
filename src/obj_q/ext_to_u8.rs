boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn to_u8(&self) -> Result<u8> {
        self.v.to_u8().ok_or(Error::ConversionFailure)
    }
}