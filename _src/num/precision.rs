use crate::num::int_i::*;

#[repr(transparent)]
pub struct Precision<const T: u8>; 

impl<const A: u8> Precision<A> where Precision<A>: PrecisionCompatibleI {

    pub const fn i128_scale() -> i128 {
        Self::scale_(10)
    }

    pub const fn u128_scale() -> u128 {
        Self::scale_(10)
    }

    const fn scale_<B>(base: B) -> B where B: IntI {
        debug_assert!(A >= 1);
        debug_assert!(A <= 38);
        match A {
            1 => base.pow(1),
            2 => base.pow(2),
            3 => base.pow(3),
            4 => base.pow(4),
            5 => base.pow(5),
            6 => base.pow(6),
            7 => base.pow(7),
            8 => base.pow(8),
            9 => base.pow(9),
            10 => base.pow(10),
            11 => base.pow(11),
            12 => base.pow(12),
            13 => base.pow(13),
            14 => base.pow(14),
            15 => base.pow(15),
            16 => base.pow(16),
            17 => base.pow(17),
            18 => base.pow(18),
            19 => base.pow(19),
            20 => base.pow(20),
            21 => base.pow(21),
            22 => base.pow(22),
            23 => base.pow(23),
            24 => base.pow(24),
            25 => base.pow(25),
            26 => base.pow(26),
            27 => base.pow(27),
            28 => base.pow(28),
            29 => base.pow(29),
            30 => base.pow(30),
            31 => base.pow(31),
            32 => base.pow(32),
            33 => base.pow(33),
            34 => base.pow(34),
            35 => base.pow(35),
            36 => base.pow(36),
            37 => base.pow(37),
            38 => base.pow(38),
            _ => panic!()
        }
    }
}

pub trait PrecisionCompatibleI {}
impl PrecisionCompatibleI for Precision<1> {}
impl PrecisionCompatibleI for Precision<2> {}
impl PrecisionCompatibleI for Precision<3> {}
impl PrecisionCompatibleI for Precision<4> {}
impl PrecisionCompatibleI for Precision<5> {}
impl PrecisionCompatibleI for Precision<6> {}
impl PrecisionCompatibleI for Precision<7> {}
impl PrecisionCompatibleI for Precision<8> {}
impl PrecisionCompatibleI for Precision<9> {}
impl PrecisionCompatibleI for Precision<10> {}
impl PrecisionCompatibleI for Precision<11> {}
impl PrecisionCompatibleI for Precision<12> {}
impl PrecisionCompatibleI for Precision<13> {}
impl PrecisionCompatibleI for Precision<14> {}
impl PrecisionCompatibleI for Precision<15> {}
impl PrecisionCompatibleI for Precision<16> {}
impl PrecisionCompatibleI for Precision<17> {}
impl PrecisionCompatibleI for Precision<18> {}
impl PrecisionCompatibleI for Precision<19> {}
impl PrecisionCompatibleI for Precision<20> {}
impl PrecisionCompatibleI for Precision<21> {}
impl PrecisionCompatibleI for Precision<22> {}
impl PrecisionCompatibleI for Precision<23> {}
impl PrecisionCompatibleI for Precision<24> {}
impl PrecisionCompatibleI for Precision<25> {}
impl PrecisionCompatibleI for Precision<26> {}
impl PrecisionCompatibleI for Precision<27> {}
impl PrecisionCompatibleI for Precision<28> {}
impl PrecisionCompatibleI for Precision<29> {}
impl PrecisionCompatibleI for Precision<30> {}
impl PrecisionCompatibleI for Precision<31> {}
impl PrecisionCompatibleI for Precision<32> {}
impl PrecisionCompatibleI for Precision<33> {}
impl PrecisionCompatibleI for Precision<34> {}
impl PrecisionCompatibleI for Precision<35> {}
impl PrecisionCompatibleI for Precision<36> {}
impl PrecisionCompatibleI for Precision<37> {}
impl PrecisionCompatibleI for Precision<38> {}