::boiler::extend!();

pub fn new<const A: u8, B>(v: B) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int::Introspection {
    Q {
        _v: v,
        _engine: default_engine::new()
    }
}

pub fn new_with_custom_engine<const A: u8, B, C>(v: B, engine: C) -> Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    Q {
        _v: v,
        _engine: engine
    }
}