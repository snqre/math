boiler::bundle!("src/core/obj_q");
boiler::expose!(
    extension,
);

use crate::common::tr_branded::*;
use crate::common::sign_introspection::*;
use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::ops::Rem;
use core::cmp::Ordering;
use num_traits::int::PrimInt;
use thiserror::Error;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B: PrimInt> where _CheckPrecision<A>: _IsPrecision {
    pub(super) v: B,
}

