boiler::bundle!("src/core/tr_sign_introspection");

mod main {
    pub trait SignIntrospection {
        fn is_signed(&self) -> bool;
    }
}

boiler::public!(
    main,
    for_i8,
    for_i16,
    for_i32,
    for_i64,
    for_i128,
    for_isize,
    for_u8,
    for_u16,
    for_u32,
    for_u64,
    for_u128,
    for_usize,
);