use crate::num::int;
use crate::num::int_introspection;
use crate::num::q;

pub enum Color<const A: u8, B, C> {
    Hex(String),
    Hsl,
    Hsla,
    Rgb(u8, u8, u8, q::Q<A, B, C>),
    Rgba
}

pub fn from_hex() -> Color {
    Color::Hex
}

pub fn from_hsl() -> Color {

}

pub fn from_hsla() -> Color {

}