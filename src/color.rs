

pub enum Color {
    Hex,
    Hsl,
    Hsla,
    Rgb,
    Rgba
}

pub fn from_hex() -> Color {
    Color::Hex
}