boiler::expose!(impl_precision);

use crate::math::precision_trait::PrecisionTrait;

#[repr(transparent)]
pub struct Precision<const A: u8>;

mod impl_precision {
    boiler::extend!();

    macro_rules! impl_precision_trait {
        // Allow trailing commas by making the comma optional
        ($($size:expr),* $(,)?) => {
            $(
                impl PrecisionTrait for Precision<{ $size }> {}
            )*
        };
    }
    
    impl_precision_trait!(
        1u8,
        2u8,
        3u8,
        4u8,
        5u8,
        6u8,
        7u8,
        8u8,
        9u8,
        10u8,
        11u8,
        12u8,
        13u8,
        14u8,
        15u8,
        16u8,
        17u8,
        18u8,
        19u8,
        20u8,
        21u8,
        22u8,
        23u8,
        24u8,
        25u8,
        26u8,
        27u8,
        28u8,
        29u8,
        30u8,
        31u8,
        32u8,
        33u8,
        34u8,
        35u8,
        36u8,
        37u8,
        38u8,
    );
}