use super::{q_util::QUtilI, *};

pub trait QI128I<T0>: Q<T0> {}

#[derive(Clone)]
#[derive(Copy)]
pub struct QI128 {
    value: i128,
    precision: u8,
}

impl QI128 {
    pub fn new(value: i128, precision: u8) -> QR<Self> {
        
    }
}

impl QI128I<QE> for QI128 {

}