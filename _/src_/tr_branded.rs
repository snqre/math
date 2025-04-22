pub mod main {
    pub trait Branded {
        fn brand(&self) -> &str;
    }
}

pub mod for_isize {
    use crate::tr_branded::main::Branded;

    impl Branded for isize {
        fn brand(&self) -> &str {
            "isize"
        }
    }
}

pub mod ext_i128 {
    use crate::tr_branded::main::Branded;

    impl Branded for i128 {
        fn brand(&self) -> &str {
            "i128"
        }
    }
}

pub mod ext_i64 {
    use crate::tr_branded::main::Branded;

    impl Branded for i64 {
        fn brand(&self) -> &str {
            "i64"
        }
    }
}

pub mod ext_i32 {
    use crate::tr_branded::main::Branded;

    impl Branded for i32 {
        fn brand(&self) -> &str {
            "i32"
        }
    }
}

pub mod ext_i16 {
    use crate::tr_branded::main::Branded;

    impl Branded for i16 {
        fn brand(&self) -> &str {
            "i16"
        }
    }
}

pub mod ext_i8 {
    use crate::tr_branded::main::Branded;

    impl Branded for i8 {
        fn brand(&self) -> &str {
            "i8"
        }
    }
}

pub mod ext_usize {
    use crate::tr_branded::main::Branded;

    impl Branded for usize {
        fn brand(&self) -> &str {
            "usize"
        }
    }
}

pub mod ext_u128 {
    use crate::tr_branded::main::Branded;

    impl Branded for u128 {
        fn brand(&self) -> &str {
            "u128"
        }
    }
}

pub mod ext_u64 {
    use crate::tr_branded::main::Branded;

    impl Branded for u64 {
        fn brand(&self) -> &str {
            "u64"
        }
    }
}

pub mod ext_u32 {
    use crate::tr_branded::main::Branded;

    impl Branded for u32 {
        fn brand(&self) -> &str {
            "u32"
        }
    }
}

pub mod ext_u16 {
    use crate::tr_branded::main::Branded;

    impl Branded for u16 {
        fn brand(&self) -> &str {
            "u16"
        }
    }
}

pub mod ext_u8 {
    use crate::tr_branded::main::Branded;

    impl Branded for u8 {
        fn brand(&self) -> &str {
            "u8"
        }
    }
}