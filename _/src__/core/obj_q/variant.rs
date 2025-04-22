boiler::extend!();












































// | Type  | Precision | Max Nominal                | Min Nominal |
    // |-------|-----------|----------------------------|-------------|
    // | Q1U8  | 1         | 2.55E+01 (~25)           | 0           |
    // | Q2U8  | 2         | 2.55E+00 (~2)            | 0           |
    // | Q3U8  | 3         | 2.55E-01 (~0.2)            | 0           |
    // | Q4U8  | 4         | 2.55E-02 (~0.02)           | 0           |
    // | Q5U8  | 5         | 2.55E-03 (~0.002)          | 0           |
    // | Q6U8  | 6         | 2.55E-04 (~0.0002)         | 0           |
    // | Q7U8  | 7         | 2.55E-05 (~0.00002)        | 0           |
    // | Q8U8  | 8         | 2.55E-06 (~0.000002)       | 0           |
    // | Q9U8  | 9         | 2.55E-07 (~0.0000002)      | 0           |
    // | Q10U8 | 10        | 2.55E-08 (~0.00000002)     | 0           |
    // | Q11U8 | 11        | 2.55E-09 (~0.000000002)    | 0           |
    // | Q12U8 | 12        | 2.55E-10 (~0.0000000002)   | 0           |
    
    // unsafe types for expanding or building on the module
    pub mod unsafe_variant_builder {
        boiler::extend!();
    
        macro_rules! _define_generic_variant {
            ($name:ident, $num:literal) => {
                pub type $name<T: PrimInt> = Q<$num, T>;
            };
        }
        
        _define_generic_variant!(Q38, 38u8);
        _define_generic_variant!(Q37, 37u8);
        _define_generic_variant!(Q36, 36u8);
        _define_generic_variant!(Q35, 35u8);
        _define_generic_variant!(Q34, 34u8);
        _define_generic_variant!(Q33, 33u8);
        _define_generic_variant!(Q32, 32u8);
        _define_generic_variant!(Q31, 31u8);
        _define_generic_variant!(Q30, 30u8);
        _define_generic_variant!(Q29, 29u8);
        _define_generic_variant!(Q28, 28u8);
        _define_generic_variant!(Q27, 27u8);
        _define_generic_variant!(Q26, 26u8);
        _define_generic_variant!(Q25, 25u8);
        _define_generic_variant!(Q24, 24u8);
        _define_generic_variant!(Q23, 23u8);
        _define_generic_variant!(Q22, 22u8);
        _define_generic_variant!(Q21, 21u8);
        _define_generic_variant!(Q20, 20u8);
        _define_generic_variant!(Q19, 19u8);
        _define_generic_variant!(Q18, 18u8);
        _define_generic_variant!(Q17, 17u8);
        _define_generic_variant!(Q16, 16u8);
        _define_generic_variant!(Q15, 15u8);
        _define_generic_variant!(Q14, 14u8);
        _define_generic_variant!(Q13, 13u8);
        _define_generic_variant!(Q12, 12u8);
        _define_generic_variant!(Q11, 11u8);
        _define_generic_variant!(Q10, 10u8);
        _define_generic_variant!(Q9, 9u8);
        _define_generic_variant!(Q8, 8u8);
        _define_generic_variant!(Q7, 7u8);
        _define_generic_variant!(Q6, 6u8);
        _define_generic_variant!(Q5, 5u8);
        _define_generic_variant!(Q4, 4u8);
        _define_generic_variant!(Q3, 3u8);
        _define_generic_variant!(Q2, 2u8);
        _define_generic_variant!(Q1, 1u8);
    }
    
    
    
    macro_rules! _define_variant {
        (
            $name:ident,
            $precision:expr,
            $usize:ty,
            $doc:expr
        ) => {
            #[doc = $doc]
            pub type $name = Q<$precision, $usize>;
        };
    }
    
    
    
    // u8
    _define_variant!(Q1U8, 1u8, u8, "***Range*** `0.00E+00` `0` to `2.55E+01` `~25`");
    _define_variant!(Q2U8, 2u8, u8, "***Range*** `0.00E+00` `0` to `2.55E+00` `~2`");
    
    
    // u16
    _define_variant!(Q1U16, 1u8, u16, "***Range*** `0.00E+00` `0` to `6.55E+03` `~6K`");
    _define_variant!(Q2U16, 1u8, u16, "***Range*** `0.00E+00` `0` to `6.55E+02` `~655`");
    _define_variant!(Q3U16, 2u8, u16, "***Range*** `0.00E+00` `0` to `6.55E+01` `~65`");
    _define_variant!(Q4U16, 3u8, u16, "***Range*** `0.00E+00` `0` to `6.55E+00` `~6`");
    
    pub mod more {
        boiler::extend!();
    
        // u8
        _define_variant!(Q3U8, 3u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-01` `~0.2`");
        _define_variant!(Q4U8, 4u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-02` `~0.002`");
        _define_variant!(Q5U8, 5u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-03` `~0.0002`");
        _define_variant!(Q6U8, 6u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-04` `~0.00002`");
        _define_variant!(Q7U8, 7u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-05` `~0.000002`");
        _define_variant!(Q8U8, 8u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-06` `~0.0000002`");
        _define_variant!(Q9U8, 9u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-07` `~0.00000002`");
        _define_variant!(Q10U8, 10u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-08` `~0.000000002`");
        _define_variant!(Q11U8, 11u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-09` `~0.0000000002`");
        _define_variant!(Q12U8, 12u8, u8, "***Range*** `0.00E+00` `0` to `2.55E-10` `~0.00000000002`");    
    
    
    }