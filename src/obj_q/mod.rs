boiler::bundle!("src/obj_q");

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
    pub(super) _v: B,
}

boiler::expose!(
    error,
    result,
    precision_range,
    precision_validation,
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

    t_q1,
    t_q1u8,
    t_q2,
    t_q2u8,
    t_q3,
    t_q4,
    t_q5,
    t_q6,
    t_q7,
    t_q8,
    t_q8,
    t_q8,
    t_q10,
    t_q11,
    t_q12,
    t_q13,
    t_q14,
    t_q15,
    t_q16,
    t_q17,
    t_q18,
    t_q19,
    t_q20,
    t_q21,
    t_q22,
    t_q23,
    t_q24,
    t_q25,
    t_q26,
    t_q27,
    t_q28,
);