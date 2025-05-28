use super::*;

pub trait Engine 
where
    Self: trig_hyperbolic_engine::TrigHyperbolicEngine,
    Self: trig_reciprocal_engine::TrigReciprocalEngine,
    Self: trig_engine::TrigEngine,
    Self: trig_conversion_engine::TrigConversionEngine,
    Self: arithmetic_engine::ArithmeticEngine,
    Self: muldiv_engine::MuldivEngine,
    Self: sign_engine::SignEngine 
{}