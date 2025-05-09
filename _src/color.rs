use crate::num::int_i;
use crate::num::int_introspection;
use crate::num::precision;
use crate::num::q;
use ::scale_info::prelude::string;

pub enum Color<const A: u8, B, C> 
where
    B: int_i::Int,
    B: int_introspection::Introspection,
    C: q::IsQEngine,
    precision::Precision<A>: precision::PrecisionCompatibleI {
    Hex(string::String),
    Hsl,
    Hsla,
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, q::Q<A, B, C>)
}



#[cfg(feature = "ink")]
pub fn from_hex<const A: u8, B, C>(code: string::String) -> Color<A, B, C> 
where
    B: int_i::Int,
    B: int_introspection::Introspection,
    C: q::IsQEngine,
    precision::Precision<A>: precision::PrecisionCompatibleI {
    Color::Hex(code)
}

pub fn from_hsl() -> Color {

}

pub fn from_hsla() -> Color {

}