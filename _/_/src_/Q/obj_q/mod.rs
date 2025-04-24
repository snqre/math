boiler::bundle!("src/q/obj_q");

use crate::common::tr_branded::*;
use crate::common::tr_sign_introspection::*;
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
pub struct Q<const A: u8, B: PrimInt> where CheckPrecision<A>: IsPrecision {
    pub(super) v: B,
}

boiler::expose!(
    constant,
    enu_error,
    ext_add,
    ext_branded,
    ext_cap_introspection,
    ext_cast,
    ext_constructor_int,
    ext_constructor,
    ext_div,
    ext_eq,
    ext_log,
    ext_mul,
    ext_muldiv,
    ext_ord,
    ext_partial_eq,
    ext_partial_ord,
    ext_prime_introspection,
    ext_rem,
    ext_sign_introspection,
    ext_sqrt,
    ext_sub,
    ext_to_f32,
    ext_to_f64,
    ext_to_i8,
    ext_to_i16,
    ext_to_i32,
    ext_to_i64,
    ext_to_i128,
    ext_to_isize,
    ext_to_u8,
    ext_to_u16,
    ext_to_u32,
    ext_to_u64,
    ext_to_u128,
    ext_to_usize,
    ext_trig_reciprocal,
    ext_trig,
    ext_trunc,
    ext_try_from_i8,
    ext_try_from_i16,
    ext_try_from_i32,
    ext_try_from_i64,
    ext_try_from_i128,
    ext_try_from_isize,
    ext_try_from_u8,
    ext_try_from_u16,
    ext_try_from_u32,
    ext_try_from_u64,
    ext_try_from_u128,
    ext_try_from_usize,
    t_result,
);
