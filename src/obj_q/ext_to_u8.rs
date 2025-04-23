boiler::extend!();

impl<const A: u8, B: PrimInt + Branded> Q<A, B> where CheckPrecision<A>: IsPrecision {
    pub fn to_u8(&self) -> Result<u8> {
        let precision: usize = A.into();
        let shift = self.v >> precision;
        shift.to_u8().ok_or(Error::ConversionFailure)
    }
}