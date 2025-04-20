boiler::extend!();

impl<const A: u8, B: PrimInt> Eq for q<A, B> where CheckPrecision<A>: _IsPrecision {}