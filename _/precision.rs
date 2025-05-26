pub const fn scale_i128<const T: u8>() -> i128 where Precision<T>: Compatible {
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
            core::hint::unreachable_unchecked()
        }
    }
}

pub const fn scale_u128<const T: u8>() -> u128 where Precision<T>: Compatible {
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
            core::hint::unreachable_unchecked()
        }
    }
}

#[repr(transparent)]
pub struct Precision<const T: u8>; 

pub trait Compatible {}
impl Compatible for Precision<1> {}
impl Compatible for Precision<2> {}
impl Compatible for Precision<3> {}
impl Compatible for Precision<4> {}
impl Compatible for Precision<5> {}
impl Compatible for Precision<6> {}
impl Compatible for Precision<7> {}
impl Compatible for Precision<8> {}
impl Compatible for Precision<9> {}
impl Compatible for Precision<10> {}
impl Compatible for Precision<11> {}
impl Compatible for Precision<12> {}
impl Compatible for Precision<13> {}
impl Compatible for Precision<14> {}
impl Compatible for Precision<15> {}
impl Compatible for Precision<16> {}
impl Compatible for Precision<17> {}
impl Compatible for Precision<18> {}
impl Compatible for Precision<19> {}
impl Compatible for Precision<20> {}
impl Compatible for Precision<21> {}
impl Compatible for Precision<22> {}
impl Compatible for Precision<23> {}
impl Compatible for Precision<24> {}
impl Compatible for Precision<25> {}
impl Compatible for Precision<26> {}
impl Compatible for Precision<27> {}
impl Compatible for Precision<28> {}
impl Compatible for Precision<29> {}
impl Compatible for Precision<30> {}
impl Compatible for Precision<31> {}
impl Compatible for Precision<32> {}
impl Compatible for Precision<33> {}
impl Compatible for Precision<34> {}
impl Compatible for Precision<35> {}
impl Compatible for Precision<36> {}
impl Compatible for Precision<37> {}
impl Compatible for Precision<38> {}