use ::num_traits as num;

pub trait Int: num::PrimInt {}

impl<T: num::PrimInt> Int for T {}