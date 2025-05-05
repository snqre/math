::boiler::extend!();

impl<const A: u8, B, C> Eq for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {}