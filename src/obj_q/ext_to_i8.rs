boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn to_i8(&self) -> Result<i8> {
        self.v.to_i8().ok_or(Error::ConversionFailure)
    }
}