::boiler::extend!();

impl q::QEngine for NaiveEngine {
    
    fn muldiv<'a, T>(&self, x: &'a T, y: &'a T, z: &'a T) -> q::QR<T>
    where
        T: int::Int,
        T: int::Introspection {
        x.checked_mul(y)
            .ok_or(q::QE::Overflow)?
            .checked_div(z)
            .ok_or(q::QE::DivByZero)
    }
}