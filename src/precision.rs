pub const SCALE_I128: [i128; 38usize] = [
    BASE_I128_.pow(1),
    BASE_I128_.pow(2),
    BASE_I128_.pow(3),
    BASE_I128_.pow(4),
    BASE_I128_.pow(5),
    BASE_I128_.pow(6),
    BASE_I128_.pow(7),
    BASE_I128_.pow(8),
    BASE_I128_.pow(9),
    BASE_I128_.pow(10),
    BASE_I128_.pow(11),
    BASE_I128_.pow(12),
    BASE_I128_.pow(13),
    BASE_I128_.pow(14),
    BASE_I128_.pow(15),
    BASE_I128_.pow(16),
    BASE_I128_.pow(17),
    BASE_I128_.pow(18),
    BASE_I128_.pow(19),
    BASE_I128_.pow(20),
    BASE_I128_.pow(21),
    BASE_I128_.pow(22),
    BASE_I128_.pow(23),
    BASE_I128_.pow(24),
    BASE_I128_.pow(25),
    BASE_I128_.pow(26),
    BASE_I128_.pow(27),
    BASE_I128_.pow(28),
    BASE_I128_.pow(29),
    BASE_I128_.pow(30),
    BASE_I128_.pow(31),
    BASE_I128_.pow(32),
    BASE_I128_.pow(33),
    BASE_I128_.pow(34),
    BASE_I128_.pow(35),
    BASE_I128_.pow(36),
    BASE_I128_.pow(37),
    BASE_I128_.pow(38)
];
const BASE_I128_: i128 = 10;

pub const SCALE_U128: [u128; 38usize] = [
    BASE_U128_.pow(1),
    BASE_U128_.pow(2),
    BASE_U128_.pow(3),
    BASE_U128_.pow(4),
    BASE_U128_.pow(5),
    BASE_U128_.pow(6),
    BASE_U128_.pow(7),
    BASE_U128_.pow(8),
    BASE_U128_.pow(9),
    BASE_U128_.pow(10),
    BASE_U128_.pow(11),
    BASE_U128_.pow(12),
    BASE_U128_.pow(13),
    BASE_U128_.pow(14),
    BASE_U128_.pow(15),
    BASE_U128_.pow(16),
    BASE_U128_.pow(17),
    BASE_U128_.pow(18),
    BASE_U128_.pow(19),
    BASE_U128_.pow(20),
    BASE_U128_.pow(21),
    BASE_U128_.pow(22),
    BASE_U128_.pow(23),
    BASE_U128_.pow(24),
    BASE_U128_.pow(25),
    BASE_U128_.pow(26),
    BASE_U128_.pow(27),
    BASE_U128_.pow(28),
    BASE_U128_.pow(29),
    BASE_U128_.pow(30),
    BASE_U128_.pow(31),
    BASE_U128_.pow(32),
    BASE_U128_.pow(33),
    BASE_U128_.pow(34),
    BASE_U128_.pow(35),
    BASE_U128_.pow(36),
    BASE_U128_.pow(37),
    BASE_U128_.pow(38)
];
const BASE_U128_: u128 = 10;

#[repr(transparent)]
pub struct Precision<const T: usize>; 

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