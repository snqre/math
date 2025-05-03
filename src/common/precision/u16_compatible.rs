use super::*;

pub trait U16Compatible {}

impl U16Compatible for Precision<1> {}
impl U16Compatible for Precision<2> {}