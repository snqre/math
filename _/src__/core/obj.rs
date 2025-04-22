// | Symbol |                                           |          |
// |--------|-------------------------------------------|----------|
// | K      | 1,000                                     | THOUSAND |
// | M      | 1,000,000                                 | MILLION  |
// | B      | 1,000,000,000                             | BILLION  |
// | T      | 1,000,000,000,000                         | TRILLION |
// | P      | 1,000,000,000,000,000                     | PETA     |
// | E      | 1,000,000,000,000,000,000                 | EXA      |
// | Z      | 1,000,000,000,000,000,000,000             | ZETTA    |
// | Y      | 1,000,000,000,000,000,000,000,000         | YOTTA    |
// | R      | 1,000,000,000,000,000,000,000,000,000     | RONNA    |
// | Q      | 1,000,000,000,000,000,000,000,000,000,000 | QUETTA   |

use thiserror::Error;

mod main {
    boiler::extend!();

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(Error)]
    pub enum Error {}

    #[repr(u8)]
    pub enum U8CompatiblePrecision {
        P1 = 1u8,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
    }

    #[repr(u8)]
    pub enum U16CompatiblePrecision {
        P1 = 1u8,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P12,
        P13,
        P14,
    }

    #[repr(u8)]
    pub enum U32CompatiblePrecision {
        P1 = 1u8,
        P2,
        P3,
        P4,
        P5,
        P6,
        P7,
        P8,
        P9,
        P10,
        P11,
        P12,
        P13,
        P14,
        P15,
        P16,
    }

    pub enum Q {
        U8(U8CompatiblePrecision, u8),
        U16(U16CompatiblePrecision, u16),
        U32(U32CompatiblePrecision, u32),
    }

    pub enum Int {
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        U128(u128),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
        
    }

    impl Q {
        pub fn new(&self) -> Int {
            match self {
                Q::U8(_, v) => Int::U8(*v),

            }
        }
    }
}











mod _internal_u {
    
}

pub type Q1U8 = _Q1U8;

enum _Q {
    Q1U8(_Q1U8),
    Q2U8(_Q2U8),
    Q3U8(u8),
    Q4U8(u8),
    Q5U8(u8),
    Q6U8(u8),
    Q7U8(u8),
    Q8U8(u8),
    Q9U8(u8),
    Q10U8(u8),
    Q11U8(u8),
    Q12U8(u8),
    Q1U16(u16),
    Q2U16(u16),
    Q3U16(u16),
    Q4U16(u16),
    Q5U16(u16),
}

#[repr(transparent)]
struct _Q1U8(u8);

#[repr(transparent)]
struct _Q2U8(u8);

impl _Q {
    pub fn max_value_representation(&self) -> f64 {
        match self {
            _Q::Q1U8(_) => 25.5f64,
            _Q::Q2U8(_) => 2.5f64,
            _Q::Q3U8(_) => 0.0f64,
            _Q::Q4U8(_) => 0.02f64,

            _ => 0.0f64,
        }
    }
    
    pub fn max_value(&self) -> u128 {

    }

    pub fn precision(&self) -> u8 {
        match self {
            _Q::Q1U8(_) => 1u8,
            _Q::Q2U8(_) => 2u8,
            _Q::Q3U8(_) => 3u8,
            _Q::Q4U8(_) => 4u8,
            _Q::Q5U8(_) => 5u8,
            _Q::Q6U8(_) => 6u8,
            _Q::Q7U8(_) => 7u8,
            _Q::Q8U8(_) => 8u8,
            _Q::Q9U8(_) => 9u8,
            _Q::Q10U8(_) => 10u8,
            _Q::Q11U8(_) => 11u8,

        }
    }
}