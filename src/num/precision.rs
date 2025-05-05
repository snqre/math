#[repr(transparent)]
pub struct Precision<const T: u8> {}

pub const fn i128_scale<const T: u8>() -> i128 {
    debug_assert!(T >= 1u8);
    debug_assert!(T <= 38u8);
    match T {
        1u8 => 10i128.pow(1u32),
        2u8 => 10i128.pow(2u32),
        3u8 => 10i128.pow(3u32),
        4u8 => 10i128.pow(4u32),
        5u8 => 10i128.pow(5u32),
        6u8 => 10i128.pow(6u32),
        7u8 => 10i128.pow(7u32),
        8u8 => 10i128.pow(8u32),
        9u8 => 10i128.pow(9u32),
        10u8 => 10i128.pow(10u32),
        11u8 => 10i128.pow(11u32),
        12u8 => 10i128.pow(12u32),
        13u8 => 10i128.pow(13u32),
        14u8 => 10i128.pow(14u32),
        15u8 => 10i128.pow(15u32),
        16u8 => 10i128.pow(16u32),
        17u8 => 10i128.pow(17u32),
        18u8 => 10i128.pow(18u32),
        19u8 => 10i128.pow(19u32),
        20u8 => 10i128.pow(20u32),
        21u8 => 10i128.pow(21u32),
        22u8 => 10i128.pow(22u32),
        23u8 => 10i128.pow(23u32),
        24u8 => 10i128.pow(24u32),
        25u8 => 10i128.pow(25u32),
        26u8 => 10i128.pow(26u32),
        27u8 => 10i128.pow(27u32),
        28u8 => 10i128.pow(28u32),
        29u8 => 10i128.pow(29u32),
        30u8 => 10i128.pow(30u32),
        31u8 => 10i128.pow(31u32),
        32u8 => 10i128.pow(32u32),
        33u8 => 10i128.pow(33u32),
        34u8 => 10i128.pow(34u32),
        35u8 => 10i128.pow(35u32),
        36u8 => 10i128.pow(36u32),
        37u8 => 10i128.pow(37u32),
        38u8 => 10i128.pow(38u32),
        _ => panic!()
    }
}

pub const fn u128_scale<const T: u8>() -> u128 {
    debug_assert!(T >= 1u8);
    debug_assert!(T <= 38u8);
    match T {
        1u8 => 10u128.pow(1u32),
        2u8 => 10u128.pow(2u32),
        3u8 => 10u128.pow(3u32),
        4u8 => 10u128.pow(4u32),
        5u8 => 10u128.pow(5u32),
        6u8 => 10u128.pow(6u32),
        7u8 => 10u128.pow(7u32),
        8u8 => 10u128.pow(8u32),
        9u8 => 10u128.pow(9u32),
        10u8 => 10u128.pow(10u32),
        11u8 => 10u128.pow(11u32),
        12u8 => 10u128.pow(12u32),
        13u8 => 10u128.pow(13u32),
        14u8 => 10u128.pow(14u32),
        15u8 => 10u128.pow(15u32),
        16u8 => 10u128.pow(16u32),
        17u8 => 10u128.pow(17u32),
        18u8 => 10u128.pow(18u32),
        19u8 => 10u128.pow(19u32),
        20u8 => 10u128.pow(20u32),
        21u8 => 10u128.pow(21u32),
        22u8 => 10u128.pow(22u32),
        23u8 => 10u128.pow(23u32),
        24u8 => 10u128.pow(24u32),
        25u8 => 10u128.pow(25u32),
        26u8 => 10u128.pow(26u32),
        27u8 => 10u128.pow(27u32),
        28u8 => 10u128.pow(28u32),
        29u8 => 10u128.pow(29u32),
        30u8 => 10u128.pow(30u32),
        31u8 => 10u128.pow(31u32),
        32u8 => 10u128.pow(32u32),
        33u8 => 10u128.pow(33u32),
        34u8 => 10u128.pow(34u32),
        35u8 => 10u128.pow(35u32),
        36u8 => 10u128.pow(36u32),
        37u8 => 10u128.pow(37u32),
        38u8 => 10u128.pow(38u32),
        _ => panic!()
    }
}

pub trait Compatible {}
impl Compatible for Precision<1u8> {}
impl Compatible for Precision<2u8> {}
impl Compatible for Precision<3u8> {}
impl Compatible for Precision<4u8> {}
impl Compatible for Precision<5u8> {}
impl Compatible for Precision<6u8> {}
impl Compatible for Precision<7u8> {}
impl Compatible for Precision<8u8> {}
impl Compatible for Precision<9u8> {}
impl Compatible for Precision<10u8> {}
impl Compatible for Precision<11u8> {}
impl Compatible for Precision<12u8> {}
impl Compatible for Precision<13u8> {}
impl Compatible for Precision<14u8> {}
impl Compatible for Precision<15u8> {}
impl Compatible for Precision<16u8> {}
impl Compatible for Precision<17u8> {}
impl Compatible for Precision<18u8> {}
impl Compatible for Precision<19u8> {}
impl Compatible for Precision<20u8> {}
impl Compatible for Precision<21u8> {}
impl Compatible for Precision<22u8> {}
impl Compatible for Precision<23u8> {}
impl Compatible for Precision<24u8> {}
impl Compatible for Precision<25u8> {}
impl Compatible for Precision<26u8> {}
impl Compatible for Precision<27u8> {}
impl Compatible for Precision<28u8> {}
impl Compatible for Precision<29u8> {}
impl Compatible for Precision<30u8> {}
impl Compatible for Precision<31u8> {}
impl Compatible for Precision<32u8> {}
impl Compatible for Precision<33u8> {}
impl Compatible for Precision<34u8> {}
impl Compatible for Precision<35u8> {}
impl Compatible for Precision<36u8> {}
impl Compatible for Precision<37u8> {}
impl Compatible for Precision<38u8> {}

pub trait PrecisionU8CompatibleI {}
impl PrecisionU8CompatibleI for Precision<1u8> {}
impl PrecisionU8CompatibleI for Precision<2u8> {}

pub trait PrecisionU16CompatibleI {}
impl PrecisionU16CompatibleI for Precision<1u8> {}
impl PrecisionU16CompatibleI for Precision<2u8> {}
impl PrecisionU16CompatibleI for Precision<3u8> {}
impl PrecisionU16CompatibleI for Precision<4u8> {}