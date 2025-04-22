boiler::extend!();

impl<const A: u8, B: PrimInt> Eq for Q<A, B> where _CheckPrecision<A>: _IsPrecision {}