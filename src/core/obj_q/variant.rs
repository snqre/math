boiler::extend!();

macro_rules! define_q_variant {
    ($module:ident, $name:ident, $num:literal) => {
        mod $module {
            boiler::extend!();

            /// todo
            /// 
            pub type $name<T: PrimInt> = Q<$num, T>;
        }
    };
}

define_q_variant!(t_q38, Q38, 38u8);
define_q_variant!(t_q37, Q37, 37u8);
define_q_variant!(t_q36, Q36, 36u8);
define_q_variant!(t_q35, Q35, 35u8);
define_q_variant!(t_q34, Q34, 34u8);
define_q_variant!(t_q33, Q33, 33u8);
define_q_variant!(t_q32, Q32, 32u8);
define_q_variant!(t_q31, Q31, 31u8);
define_q_variant!(t_q30, Q30, 30u8);
define_q_variant!(t_q29, Q29, 29u8);
define_q_variant!(t_q28, Q28, 28u8);
define_q_variant!(t_q27, Q27, 27u8);
define_q_variant!(t_q26, Q26, 26u8);
define_q_variant!(t_q25, Q25, 25u8);
define_q_variant!(t_q24, Q24, 24u8);
define_q_variant!(t_q23, Q23, 23u8);
define_q_variant!(t_q22, Q22, 22u8);
define_q_variant!(t_q21, Q21, 21u8);
define_q_variant!(t_q20, Q20, 20u8);
define_q_variant!(t_q19, Q19, 19u8);
define_q_variant!(t_q18, Q18, 18u8);
define_q_variant!(t_q17, Q17, 17u8);
define_q_variant!(t_q16, Q16, 16u8);
define_q_variant!(t_q15, Q15, 15u8);
define_q_variant!(t_q14, Q14, 14u8);
define_q_variant!(t_q13, Q13, 13u8);
define_q_variant!(t_q12, Q12, 12u8);
define_q_variant!(t_q11, Q11, 11u8);
define_q_variant!(t_q10, Q10, 10u8);
define_q_variant!(t_q9, Q9, 9u8);
define_q_variant!(t_q8, Q8, 8u8);
define_q_variant!(t_q7, Q7, 7u8);
define_q_variant!(t_q6, Q6, 6u8);
define_q_variant!(t_q5, Q5, 5u8);
define_q_variant!(t_q4, Q4, 4u8);
define_q_variant!(t_q3, Q3, 3u8);
define_q_variant!(t_q2, Q2, 2u8);
define_q_variant!(t_q1, Q1, 1u8);

boiler::public!(
    t_q38,
    t_q37,
    t_q36,
    t_q35,
    t_q34,
    t_q33,
    t_q32,
    t_q31,
    t_q30,
    t_q29,
    t_q28,
    t_q27,
    t_q26,
    t_q25,
    t_q24,
    t_q23,
    t_q22,
    t_q21,
    t_q20,
    t_q19,
    t_q18,
    t_q17,
    t_q16,
    t_q15,
    t_q14,
    t_q13,
    t_q12,
    t_q11,
    t_q10,
    t_q9,
    t_q8,
    t_q7,
    t_q6,
    t_q5,
    t_q4,
    t_q3,
    t_q2,
    t_q1,
);