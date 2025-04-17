#[allow(unused_imports)]
pub mod prelude {
    pub use crate::m_color::core::*;
    pub use crate::m_color::ext_hex::*;
    pub use crate::m_color::ext_rgb::*;
    pub use crate::m_color::ext_rgba::*;
}

pub mod core {
    use crate::m_k::prelude::*;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;
    use num_traits::Unsigned;
    use thiserror::Error;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(Error)]
    pub enum Error {
        #[error("")]
        KError(#[from] crate::m_k::core::Error),
        #[error("")]
        InvalidHexCode,
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Color<const A: u8, B: PrimInt + Branded + Unsigned> {
        pub(super) _slot: _Slot<A, B>,
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub(super) enum _Slot<const A: u8, B: PrimInt + Branded + Unsigned> {
        Hex(String),
        Hsl(u16, K<A, B>, K<A, B>),
        Hsla(u16, K<A, B>, K<A, B>, K<A, B>),
        Rgb(u8, u8, u8),
        Rgba(u8, u8, u8, K<A, B>),
    }
}

pub mod ext_hex {
    use crate::m_color::core::Result;
    use crate::m_color::core::Error;
    use crate::m_color::core::Color;
    use crate::m_color::core::_Slot;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;
    use num_traits::Unsigned;
    
    impl<const A: u8, B: PrimInt + Branded + Unsigned> Color<A, B> {
        pub fn from_hex(code: &str) -> Result<Self> {
            let mut code: String = code.to_owned();
            code = code
                .trim()
                .replace(" ", "");
            if code.is_empty() {
                return Err(Error::InvalidHexCode)
            }
            if !code.starts_with("#") {
                code = format!("#{}", code);
            }
            if code.len() != 7usize {
                return Err(Error::InvalidHexCode)
            }
            let result: Self = Self {
                _slot: _Slot::Hex(code),
            };
            Ok(result)
        }

        pub fn to_hex(&self) -> Result<String> {
            match &self._slot {
                _Slot::Hex(code) => Ok(code.to_owned()),
                _Slot::Hsl(h, s, l)  => {
                    let (r, g, b) = Self::_to_rgb_from_hsl(h, s, l)?;
                    Ok(Self::_to_hex_from_rgb(r, g, b))
                },
                _Slot::Hsla(h, s, l, _) => {
                    let (r, g, b) = Self::_to_rgb_from_hsl(h, s, l)?;
                    Ok(Self::_to_hex_from_rgb(r, g, b))
                },
                _Slot::Rgb(r, g, b) => Ok(Self::_to_hex_from_rgb(*r, *g, *b)),
                _Slot::Rgba(r, g, b, _) => Ok(Self::_to_hex_from_rgb(*r, *g, *b)),
            }
        }

        pub(super) fn _to_hex_from_rgb(r: u8, g: u8, b: u8) -> String {
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        }
    }
}

pub mod ext_hsl {
    use crate::m_color::core::Color;
    use crate::m_color::core::Result;
    use crate::m_color::core::_Slot;
    use crate::m_k::core::K;
    use crate::m_k::ext_constructor::k;
    use crate::prelude::Branded;
    use num_traits::int::PrimInt;
    use num_traits::Unsigned;

    impl<const A: u8, B: PrimInt + Branded + Unsigned> Color<A, B> {
        pub fn from_hsl(h: u16, s: K<A, B>, l: K<A, B>) -> Self {
            Self {
                _slot: _Slot::Hsl(h, s, l),
            }
        }

        pub fn to_hsl(&self) -> Result<(u16, K<A, B>, K<A, B>)> {
            match &self._slot {
                _Slot::Hex(code) => {
                    let (r, g, b) = Self::_to_rgb_from_hex(code);
                    Self::_to_hsl_from_rgb(r, g, b)
                },
                _Slot::Hsl(h, s, l) => Ok((*h, s.clone(), l.clone())),
                _Slot::Hsla(h, s, l, _) => Ok((*h, s.clone(), l.clone())),
                _Slot::Rgb(r, g, b) => Self::_to_hsl_from_rgb(*r, *g, *b),
                _Slot::Rgba(r, g, b, _) => Self::_to_hsl_from_rgb(*r, *g, *b),
            }
        }

        pub(super) fn _to_hsl_from_rgb(r: u8, g: u8, b: u8) -> Result<(u16, K<A, B>, K<A, B>)> {
            let xv2: B = B::from(2u8).unwrap();
            let xv2: K<A, B> = k(xv2);
            let xv0: B = B::from(0u8).unwrap();
            let xv0: K<A, B> = k(xv0);
            let r: B = B::from(r).unwrap();
            let r: K<A, B> = k(r);
            let g: B = B::from(g).unwrap();
            let g: K<A, B> = k(g);
            let b: B = B::from(b).unwrap();
            let b: K<A, B> = k(b);
            let max: K<A, B> = r.max(g.max(b));
            let min: K<A, B> = r.min(g.min(b));
            let delta: K<A, B> = (max - min)?;
            let l: K<A, B> = (max + min)?;
            let l: K<A, B> = (l / xv2)?;
            let mut s: K<A, B> = xv0;
            let mut h: K<A, B> = xv0;
            if delta != xv0 {
                let xv0_5: B = B::from()
            }
        }
    }
}

pub mod ext_rgb {
    use crate::m_color::core::*;
    use crate::m_k::core::K;
    use crate::m_k::ext_constructor::k;
    use crate::m_k::ext_constructor_::k_int;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;
    use num_traits::Unsigned;

    impl<const A: u8, B: PrimInt + Branded + Unsigned> Color<A, B> {
        pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
            Self {
                _slot: _Slot::Rgb(r, g, b),
            }
        }

        pub fn to_rgb(&self) -> Result<(u8, u8, u8)> {
            match &self._slot {
                _Slot::Hex(code) => Ok(Self::_to_rgb_from_hex(code)),
                _Slot::Hsl(h, s, l) => Self::_to_rgb_from_hsl(h, s, l),
                _Slot::Hsla(h, s, l, _) => Self::_to_rgb_from_hsl(h, s, l),
                _Slot::Rgb(r, g, b) => Ok((*r, *g, *b)),
                _Slot::Rgba(r, g, b, _) => Ok((*r, *g, *b)),
            }
        }

        pub(super) fn _to_rgb_from_hex(code: &str) -> (u8, u8, u8) {
            let r: u8 = u8::from_str_radix(&code[0..2], 16).unwrap();
            let g: u8 = u8::from_str_radix(&code[2..4], 16).unwrap();
            let b: u8 = u8::from_str_radix(&code[4..6], 16).unwrap();
            (r, g, b)
        }

        pub(super) fn _to_rgb_from_hsl(h: &u16, s: &K<A, B>, l: &K<A, B>) -> Result<(u8, u8, u8)> {
            let k360: K<A, B> = k_int::<2u8, u16, A, B>(36000u16)?;
            let k100: K<A, B> = k_int::<2u8, u16, A, B>(10000u16)?;
            let k6: K<A, B> = k_int::<2u8, u16, A, B>(600u16)?;
            let k5: K<A, B> = k_int::<2u8, u16, A, B>(500u16)?;
            let k4: K<A, B> = k_int::<2u8, u16, A, B>(400u16)?;
            let k3: K<A, B> = k_int::<2u8, u16, A, B>(300u16)?;
            let k2: K<A, B> = k_int::<2u8, u16, A, B>(200u16)?;
            let k1: K<A, B> = k_int::<2u8, u16, A, B>(100u16)?;
            let k0: K<A, B> = k_int::<2u8, u16, A, B>(0u16)?;
            let h: B = B::from(*h).unwrap();
            

            let h: K<A, B> = k(h);
            let s: K<A, B> = (*s / k100)?;
            let l: K<A, B> = (*l / k100)?;
            let c: K<A, B> = (k2 * l)?;
            let c: K<A, B> = (c - k1)?;
            let c: K<A, B> = (c * s)?;
            let h_prime: K<A, B> = (h & k6)?;
            let h_prime: K<A, B> = (h_prime / k360)?;


            let c: K<A, B> = (((xv2 * l)? - xv1)? * s)?;
            let h_prime: K<A, B> = ((h * k6)? / k360)?;
            let x: K<A, B> = (c * ((h_prime % xv2)? - xv1)?)?;
            let (r, g, b) = if h_prime < xv1 {
                (c, x, xv0)
            } else if h_prime < xv2 {
                (x, c, xv0)
            } else if h_prime < xv3 {
                (xv0, c, x)
            } else if h_prime < xv4 {
                (xv0, x, c)
            } else if h_prime < xv5 {
                (x, xv0, c)
            } else {
                (c, xv0, x)
            };
            let m: K<A, B> = ((l - c)? / xv2)?;
            let r: K<A, B> = (r + m)?;
            let g: K<A, B> = (g + m)?;
            let b: K<A, B> = (b + m)?;
            let r: u8 = r.to_u8()?;
            let g: u8 = g.to_u8()?;
            let b: u8 = b.to_u8()?;
            Ok((r, g, b))
        }
    }
}

pub mod ext_rgba {
    use crate::m_color::core::*;
    use crate::m_k::core::K;
    use crate::m_k::ext_constructor::k;
    use crate::extension::tr_branded::Branded;
    use num_traits::int::PrimInt;
    use num_traits::Unsigned;

    impl<const A: u8, B: PrimInt + Branded + Unsigned> Color<A, B> {
        pub fn from_rgba(r: u8, g: u8, b: u8, a: K<A, B>) -> Self {
            Self {
                _slot: _Slot::Rgba(r, g, b, a),
            }
        }

        pub fn to_rgba(&self) -> Result<(u8, u8, u8, K<A, B>)> {
            match &self._slot {
                _Slot::Hex(code) => {
                    let (r, g, b) = Self::_to_rgb_from_hex(code);
                    let a: B = B::from(1u8).unwrap();
                    let a: K<A, B> = k(a);
                    Ok((r, g, b, a))
                },
                _Slot::Hsl(h, s, l) | _Slot::Hsla(h, s, l, _) => {
                    let (r, g, b) = Self::_to_rgb_from_hsl(h, s, l)?;
                    let a: K<A, B> = if let _Slot::Hsla(_, _, _, a) = &self._slot {
                        a.to_owned()
                    } else {
                        let a: B = B::from(1u8).unwrap();
                        let a: K<A, B> = k(a);
                        a
                    };
                    Ok((r, g, b, a))
                },
                _Slot::Rgb(r, g, b) => {
                    let a: B = B::from(1u8).unwrap();
                    let a: K<A, B> = k(a);
                    Ok((*r, *g, *b, a))
                },
                _Slot::Rgba(r, g, b, a) => Ok((*r, *g, *b, *a)),
            }
        }
    }
}