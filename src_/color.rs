#![allow(unused)]

pub mod main {
    use crate::k::main::K;
    use crate::tr_branded::main::Branded;
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
        KError(#[from] crate::k::main::Error),
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
    use crate::color::main::Result;
    use crate::color::main::Error;
    use crate::color::main::Color;
    use crate::color::main::_Slot;
    use crate::tr_branded::main::Branded;
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
    use crate::color::main::Color;
    use crate::color::main::Result;
    use crate::color::main::_Slot;
    use crate::k::main::K;
    use crate::k::ext_int_constructor::k_int;
    use crate::tr_branded::main::Branded;
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
                    let code: &str = code.as_str();
                    let (r, g, b) = Self::_to_rgb_from_hex(code);
                    let hsl: (u16, K<A, B>, K<A, B>) = Self::_to_hsl_from_rgb(r, g, b)?;
                    Ok(hsl)
                },
                _Slot::Hsl(h, s, l) => {
                    let h: u16 = *h;
                    let s: K<A, B> = *s;
                    let l: K<A, B> = *l;
                    let hsl: (u16, K<A, B>, K<A, B>) = (h, s, l);
                    Ok(hsl)
                },
                _Slot::Hsla(h, s, l, _) => {
                    let h: u16 = *h;
                    let s: K<A, B> = *s;
                    let l: K<A, B> = *l;
                    let hsl: (u16, K<A, B>, K<A, B>) = (h, s, l);
                    Ok(hsl)
                },
                _Slot::Rgb(r, g, b) => {
                    let r: u8 = *r;
                    let g: u8 = *g;
                    let b: u8 = *b;
                    let hsl: (u16, K<A, B>, K<A, B>) = Self::_to_hsl_from_rgb(r, g, b)?;
                    Ok(hsl)
                },
                _Slot::Rgba(r, g, b, _) => {
                    let r: u8 = *r;
                    let g: u8 = *g;
                    let b: u8 = *b;
                    let hsl: (u16, K<A, B>, K<A, B>) = Self::_to_hsl_from_rgb(r, g, b)?;
                    Ok(hsl)
                },
            }
        }

        pub(super) fn _to_hsl_from_rgb(r: u8, g: u8, b: u8) -> Result<(u16, K<A, B>, K<A, B>)> {
            let k: _ = |v: u16| -> Result<K<A, B>> {
                Ok(k_int::<2u8, u16, A, B>(v)?)
            };
            let r: K<A, B> = (k_int::<0u8, u8, A, B>(r)? / k(25500u16)?)?;
            let g: K<A, B> = (k_int::<0u8, u8, A, B>(g)? / k(25500u16)?)?; 
            let b: K<A, B> = (k_int::<0u8, u8, A, B>(b)? / k(25500u16)?)?;
            let max: K<A, B> = r.max(g).max(b);
            let min: K<A, B> = r.min(g).min(b);
            let delta: K<A, B> = (max - min)?;
            let l: K<A, B> = ((max + min)? / k(200u16)?)?;
            let mut s: K<A, B> = k(0u16)?;
            let mut h: K<A, B> = k(0u16)?;
            if delta != k(0u16)? {
                s = if l > k(50u16)? {
                    (delta / ((k(200u16)? - max)? - min)?)?
                } else {
                    (delta / (max + min)?)?
                };
                if max == r {
                    h = ((g - b)? / delta)?
                } else if max == g {
                    h = (k(200u16)? + ((b - r)? / delta)?)?
                } else {
                    h = (k(400u16)? + ((r - g)? / delta)?)?
                }
                h = (h * k(6000u16)?)?;
                if h < k(0u16)? {
                    h = (h + k(36000u16)?)?;
                }
            }
            let h: u16 = h.to_u16()?;
            let s: K<A, B> = (s * k(10000u16)?)?;
            let l: K<A, B> = (l * k(10000u16)?)?;
            Ok((h, s, l))
        }
    }
}

pub mod ext_hsla {
    use crate::color::main::Result;
    use crate::color::main::Color;
    use crate::color::main::_Slot;
    use crate::k::main::K;
    use crate::k::ext_int_constructor::k_int;
    use crate::tr_branded::main::Branded;
    use num_traits::int::PrimInt;
    use num_traits::Unsigned;

    impl<const A: u8, B: PrimInt + Branded + Unsigned> Color<A, B> {
        pub fn from_hsla(h: u16, s: K<A, B>, l: K<A, B>, a: K<A, B>) -> Self {
            Self {
                _slot: _Slot::Hsla(h, s, l, a),
            }
        }

        #[allow(clippy::type_complexity)]
        pub fn to_hsla(&self) -> Result<(u16, K<A, B>, K<A, B>, K<A, B>)> {
            match &self._slot {
                _Slot::Hex(code) => {
                    let (r, g, b) = Self::_to_rgb_from_hex(code);
                    let (h, s, l) = Self::_to_hsl_from_rgb(r, g, b)?;
                    let a: K<A, B> = k_int::<2u8, u16, A, B>(100u16)?;
                    let hsla: (u16, K<A, B>, K<A, B>, K<A, B>) = (h, s, l, a);
                    Ok(hsla)
                },
                _Slot::Hsl(h, s, l) => {
                    let h: u16 = *h;
                    let s: K<A, B> = *s;
                    let l: K<A, B> = *l;
                    let a: K<A, B> = k_int::<2u8, u16, A, B>(100u16)?;
                    let hsla: (u16, K<A, B>, K<A, B>, K<A, B>) = (h, s, l, a);
                    Ok(hsla)
                },
                _Slot::Hsla(h, s, l, a) => {
                    let h: u16 = *h;
                    let s: K<A, B> = *s;
                    let l: K<A, B> = *l;
                    let a: K<A, B> = *a;
                    let hsla: (u16, K<A, B>, K<A, B>, K<A, B>) = (h, s, l, a);
                    Ok(hsla)
                },
                _Slot::Rgb(r, g, b) => {
                    let r: u8 = *r;
                    let g: u8 = *g;
                    let b: u8 = *b;
                    let (h, s, l) = Self::_to_hsl_from_rgb(r, g, b)?;
                    let a: K<A, B> = k_int::<2u8, u16, A, B>(100u16)?;
                    let hsla: (u16, K<A, B>, K<A, B>, K<A, B>) = (h, s, l, a);
                    Ok(hsla)
                },
                _Slot::Rgba(r, g, b, a) => {
                    let r: u8 = *r;
                    let g: u8 = *g;
                    let b: u8 = *b;
                    let a: K<A, B> = *a;
                    let (h, s, l) = Self::_to_hsl_from_rgb(r, g, b)?;
                    let a: K<A, B> = k_int::<2u8, u16, A, B>(100u16)?;
                    let hsla: (u16, K<A, B>, K<A, B>, K<A, B>) = (h, s, l, a);
                    Ok(hsla)
                },
            }
        }
    }
}

pub mod ext_rgb {
    use crate::color::main::*;
    use crate::k::main::K;
    use crate::k::ext_int_constructor::k_int;
    use crate::tr_branded::main::Branded;
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
            let code: &str = code.trim_start_matches("#");
            let r: u8 = u8::from_str_radix(&code[0..2], 16).unwrap();
            let g: u8 = u8::from_str_radix(&code[2..4], 16).unwrap();
            let b: u8 = u8::from_str_radix(&code[4..6], 16).unwrap();
            (r, g, b)
        }

        pub(super) fn _to_rgb_from_hsl(h: &u16, s: &K<A, B>, l: &K<A, B>) -> Result<(u8, u8, u8)> {
            let k: _ = |v: u16| -> Result<K<A, B>> {
                Ok(k_int::<2u8, u16, A, B>(v)?)
            };
            let h: u16 = *h;
            let h: K<A, B> = k_int::<0u8, u16, A, B>(h)?;
            let s: K<A, B> = *s;
            let s: K<A, B> = (s / k(10000u16)?)?;
            let l: K<A, B> = *l;
            let l: K<A, B> = (l / k(10000u16)?)?;
            let c: K<A, B> = (((k(200u16)? * l)? - k(100u16)?)? * s)?;
            let h_prime: K<A, B> = ((h * k(600u16)?)? / k(360u16)?)?;
            let x: K<A, B> = (c * ((h_prime % k(200u16)?)? - k(100u16)?)?)?;
            let (r, g, b) = if h_prime < k(100u16)? {
                (c, x, k(0)?)
            } else if h_prime < k(200u16)? {
                (x, c, k(0)?)
            } else if h_prime < k(300u16)? {
                (k(0)?, c, x)
            } else if h_prime < k(400u16)? {
                (k(0)?, x, c)
            } else if h_prime < k(500u16)? {
                (x, k(0)?, c)
            } else {
                (c, k(0)?, x)
            };
            let m: K<A, B> = ((l - c)? / k(200u16)?)?;
            let r: u8 = (r + m)?.to_u8()?;
            let g: u8 = (g + m)?.to_u8()?;
            let b: u8 = (b + m)?.to_u8()?;
            Ok((r, g, b))
        }
    }
}

pub mod ext_rgba {
    use crate::color::main::*;
    use crate::k::main::K;
    use crate::k::ext_constructor::k;
    use crate::tr_branded::main::Branded;
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

pub mod ext_partial_eq {
    use crate::color::main::Color;
    use crate::tr_branded::main::Branded;
    use num_traits::int::PrimInt;
    use num_traits::Unsigned;

    impl<const A: u8, B: PrimInt + Branded + Unsigned> PartialEq for Color<A, B> {    
        fn eq(&self, other: &Self) -> bool {
            if let (Ok(rgba_0), Ok(rgba_1)) = (self.to_rgba(), other.to_rgba()) {
                let (r_0, g_0, b_0, a_0) = rgba_0;
                let (r_1, g_1, b_1, a_1) = rgba_1;
                return r_0 == r_1 && g_0 == g_1 && b_0 == b_1 && a_0 == a_1
            }
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::color::main::*;

    #[test]
    fn to_hex() {
        let hex = Color::<2u8, u128>::from_rgb(81u8, 187u8, 254u8).to_hex().unwrap();
        let hex_ok = "#51BBFE";
        assert_eq!(hex, hex_ok);
    }

    #[test]
    fn to_rgb() {
        let rgb = Color::<2u8, u128>::from_hex("#51BBFE").unwrap().to_rgb().unwrap();
        let rgb_ok = (81u8, 187u8, 254u8);
        assert_eq!(rgb, rgb_ok);
    }
}