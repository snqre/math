use crate::q;
use crate::int;
use crate::precision;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("")]
    Q(#[from] q::Error)
}

pub enum Color<const A: usize, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine,
    precision::Precision<A>: precision::Compatible {
    #[cfg(feature = "std")]
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, q::Q<A, B, C>),
    Hsl(u16, q::Q<A, B, C>, q::Q<A, B, C>),
    Hsla(u16, q::Q<A, B, C>, q::Q<A, B, C>, q::Q<A, B, C>)
}

#[cfg(feature = "std")]
pub fn from_hex(code: &str) -> Color {
    
}

pub fn from_rgb<const A: usize, B, C>(r: u8, g: u8, b: u8) -> Color<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine,
    precision::Precision<A>: precision::Compatible {
    Color::Rgb(r, g, b)
}

pub fn from_rgba<const A: usize, B, C>(r: u8, g: u8, b: u8, a: q::Q<A, B, C>) -> Color<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine,
    precision::Precision<A>: precision::Compatible {
    Color::Rgba(r, g, b, a)
}

toga::blockset! {
    impl<const A: usize, B, C> Color<A, B, C>
    where
        B: int::Int,
        B: int::Introspection,
        C: q::Engine,
        precision::Precision<A>: precision::Compatible;

    pub fn to_rgba(&self) -> Result<(u8, u8, u8, q::Q<A, B, C>)> {
        
    }

    pub fn to_rgb(&self) -> Result<(u8, u8, u8)> {
        match self {
            #[cfg(feature = "std")]
            Color::Hex(code) => {
                let (r, g, b) = match code.len() {
                    3 => {
                        let r = &code[0..1].repeat(2);
                        let r: u8 = u8::from_str_radix(r, 16).unwrap();
                        let g = &code[1..2].repeat(2);
                        let g: u8 = u8::from_str_radix(g, 16).unwrap();
                        let b = &code[2..3].repeat(2);
                        let b = u8::from_str_radix(b, 16).unwrap();
                        (r, g, b)
                    },
                    6 => {
                        let r = u8::from_str_radix(&code[0..1], 16).unwrap();
                        let g = u8::from_str_radix(&code[2..4], 16).unwrap();
                        let b = u8::from_str_radix(&code[4..6], 16).unwrap();
                        (r, g, b)
                    },
                    _ => panic!()
                };
                Ok((r, g, b))
            },
            Color::Rgb(r, g, b) | Color::Rgba(r, g, b, _) => Ok((*r, *g, *b)),
            Color::Hsl(h, s, l) | Color::Hsla(h, s, l, _) => {
                let v = move |v: u16| -> Result<q::Q<A, B, C>> {
                    Ok(q::from_int_with_engine_from(v, *l)?)
                };
                let h: q::Q<A, B, C> = v(*h)?;
                let h: q::Q<A, B, C> = (h / v(36000)?)?;
                let s: q::Q<A, B, C> = *s;
                let l: q::Q<A, B, C> = *l;
                let k = if l < v(50)? {
                    (l * (v(100)? + s)?)?
                } else {
                    ((l + s)? - (l * s)?)?
                };
                let p = ((v(200)? * l)? - k)?;
                let hue_to_rgb = |p: q::Q<A, B, C>, k: q::Q<A, B, C>, mut t: q::Q<A, B, C>| -> Result<q::Q<A, B, C>> {
                    if t < v(0)? {
                        t = (t + v(100)?)?;
                    }
                    if t > v(100)? {
                        t = (t - v(100)?)?;
                    }
                    if t < (v(100)? / v(600)?)? {
                        Ok((p + (((k - p)? * v(600)?)? * t)?)?)
                    } else if t < v(50)? {
                        Ok(k)
                    } else if t < (v(200)? / v(300)?)? {
                        Ok((p + (((k - p)? * ((v(200)? / v(300)?)? - t)?)? * v(600)?)?)?)
                    } else {
                        Ok(p)
                    }
                };
                let offset = (v(100)? / v(300)?)?;
                let r = hue_to_rgb(p, k, (h + offset)?)?;
                let g = hue_to_rgb(p, k, h)?;
                let b = hue_to_rgb(p, k, (h - offset)?)?;
                Ok((
                    (r * v(25500)?)?.to_u8().unwrap(),
                    (g * v(25500)?)?.to_u8().unwrap(),
                    (b * v(25500)?)?.to_u8().unwrap()
                ))
            }
        }
    }
}

#[cfg(feature = "std")]
toga::blockset! {
    impl<const A: usize, B, C> Color<A, B, C>
    where
        B: int::Int,
        B: int::Introspection,
        C: q::Engine,
        precision::Precision<A>: precision::Compatible;

    pub fn to_hex(&self) -> String {}
}