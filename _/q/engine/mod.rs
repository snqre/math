use super::*;

mod base;
mod i_arithmetic_algo;
mod muldiv;
mod sign;
mod trig_conversion;
mod trig_hyperbolic;
mod trig_reciprocal;
mod trig;

pub trait Handler
where 
	Self: base::Handler,
	Self: muldiv::Handler,
	Self: pi::Accessor,
	Self: scale::Accessor,
	Self: sign::Handler,
	Self: trig_conversion::Handler,
	Self: trig_hyperbolic::Handler,
	Self: trig_reciprocal::Handler,
	Self: trig::Handler 
{}


pub struct Default;

impl Handler for Default {}