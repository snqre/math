use super::*;

#[inline]
pub fn get<const A: u8, B>() -> B where B: int::Int {
    if B::IS_SIGNED {
        let scale: i128 = a::<A>();
        let scale: B = unsafe {
            B::from(scale).unwrap_unchecked()
        };
        return scale
    }
    let scale: u128 = b::<A>();
    let scale: B = unsafe {
        B::from(scale).unwrap_unchecked()
    };
    scale
}

const fn a<const T: u8>() -> i128 {
    assert!(T >= 1);
    assert!(T <= 38);
    const BASE: i128 = 10;
    match T {
        1 => BASE.pow(1),
        2 => BASE.pow(2),
        3 => BASE.pow(3),
        4 => BASE.pow(4),
        5 => BASE.pow(5),
        6 => BASE.pow(6),
        7 => BASE.pow(7),
        8 => BASE.pow(8),
        9 => BASE.pow(9),
        10 => BASE.pow(10),
        11 => BASE.pow(11),
        12 => BASE.pow(12),
        13 => BASE.pow(13),
        14 => BASE.pow(14),
        15 => BASE.pow(15),
        16 => BASE.pow(16),
        17 => BASE.pow(17),
        18 => BASE.pow(18),
        19 => BASE.pow(19),
        20 => BASE.pow(20),
        21 => BASE.pow(21),
        22 => BASE.pow(22),
        23 => BASE.pow(23),
        24 => BASE.pow(24),
        25 => BASE.pow(25),
        26 => BASE.pow(26),
        27 => BASE.pow(27),
        28 => BASE.pow(28),
        29 => BASE.pow(29),
        30 => BASE.pow(30),
        31 => BASE.pow(31),
        32 => BASE.pow(32),
        33 => BASE.pow(33),
        34 => BASE.pow(34),
        35 => BASE.pow(35),
        36 => BASE.pow(36),
        37 => BASE.pow(37),
        38 => BASE.pow(38),
        _ => unsafe {
            ::core::hint::unreachable_unchecked()
        }
    }
}

const fn b<const T: u8>() -> u128 {
    assert!(T >= 1);
    assert!(T <= 38);
    const BASE: u128 = 10;
    match T {
        1 => BASE.pow(1),
        2 => BASE.pow(2),
        3 => BASE.pow(3),
        4 => BASE.pow(4),
        5 => BASE.pow(5),
        6 => BASE.pow(6),
        7 => BASE.pow(7),
        8 => BASE.pow(8),
        9 => BASE.pow(9),
        10 => BASE.pow(10),
        11 => BASE.pow(11),
        12 => BASE.pow(12),
        13 => BASE.pow(13),
        14 => BASE.pow(14),
        15 => BASE.pow(15),
        16 => BASE.pow(16),
        17 => BASE.pow(17),
        18 => BASE.pow(18),
        19 => BASE.pow(19),
        20 => BASE.pow(20),
        21 => BASE.pow(21),
        22 => BASE.pow(22),
        23 => BASE.pow(23),
        24 => BASE.pow(24),
        25 => BASE.pow(25),
        26 => BASE.pow(26),
        27 => BASE.pow(27),
        28 => BASE.pow(28),
        29 => BASE.pow(29),
        30 => BASE.pow(30),
        31 => BASE.pow(31),
        32 => BASE.pow(32),
        33 => BASE.pow(33),
        34 => BASE.pow(34),
        35 => BASE.pow(35),
        36 => BASE.pow(36),
        37 => BASE.pow(37),
        38 => BASE.pow(38),
        _ => unsafe {
            ::core::hint::unreachable_unchecked()
        }
    }
}