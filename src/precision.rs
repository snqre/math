pub const SCALE_I128: [i128; 39] = [
    _BASE_I128.pow(0), // should not be used.
    _BASE_I128.pow(1),
    _BASE_I128.pow(2),
    _BASE_I128.pow(3),
    _BASE_I128.pow(4),
    _BASE_I128.pow(5),
    _BASE_I128.pow(6),
    _BASE_I128.pow(7),
    _BASE_I128.pow(8),
    _BASE_I128.pow(9),
    _BASE_I128.pow(10),
    _BASE_I128.pow(11),
    _BASE_I128.pow(12),
    _BASE_I128.pow(13),
    _BASE_I128.pow(14),
    _BASE_I128.pow(15),
    _BASE_I128.pow(16),
    _BASE_I128.pow(17),
    _BASE_I128.pow(18),
    _BASE_I128.pow(19),
    _BASE_I128.pow(20),
    _BASE_I128.pow(21),
    _BASE_I128.pow(22),
    _BASE_I128.pow(23),
    _BASE_I128.pow(24),
    _BASE_I128.pow(25),
    _BASE_I128.pow(26),
    _BASE_I128.pow(27),
    _BASE_I128.pow(28),
    _BASE_I128.pow(29),
    _BASE_I128.pow(30),
    _BASE_I128.pow(31),
    _BASE_I128.pow(32),
    _BASE_I128.pow(33),
    _BASE_I128.pow(34),
    _BASE_I128.pow(35),
    _BASE_I128.pow(36),
    _BASE_I128.pow(37),
    _BASE_I128.pow(38)
];

const _BASE_I128: i128 = 10;

pub const SCALE_U128: [u128; 39] = [
    _BASE_U128.pow(0), // should not be used.
    _BASE_U128.pow(1),
    _BASE_U128.pow(2),
    _BASE_U128.pow(3),
    _BASE_U128.pow(4),
    _BASE_U128.pow(5),
    _BASE_U128.pow(6),
    _BASE_U128.pow(7),
    _BASE_U128.pow(8),
    _BASE_U128.pow(9),
    _BASE_U128.pow(10),
    _BASE_U128.pow(11),
    _BASE_U128.pow(12),
    _BASE_U128.pow(13),
    _BASE_U128.pow(14),
    _BASE_U128.pow(15),
    _BASE_U128.pow(16),
    _BASE_U128.pow(17),
    _BASE_U128.pow(18),
    _BASE_U128.pow(19),
    _BASE_U128.pow(20),
    _BASE_U128.pow(21),
    _BASE_U128.pow(22),
    _BASE_U128.pow(23),
    _BASE_U128.pow(24),
    _BASE_U128.pow(25),
    _BASE_U128.pow(26),
    _BASE_U128.pow(27),
    _BASE_U128.pow(28),
    _BASE_U128.pow(29),
    _BASE_U128.pow(30),
    _BASE_U128.pow(31),
    _BASE_U128.pow(32),
    _BASE_U128.pow(33),
    _BASE_U128.pow(34),
    _BASE_U128.pow(35),
    _BASE_U128.pow(36),
    _BASE_U128.pow(37),
    _BASE_U128.pow(38)
];

const _BASE_U128: u128 = 10;

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