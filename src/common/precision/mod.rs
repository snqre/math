#[repr(transparent)]
pub struct Precision<const T: u8>;

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

pub trait I8Compatible {}
impl I8Compatible for Precision<1> {}
impl I8Compatible for Precision<2> {}

pub trait I16Compatible {}
impl I16Compatible for Precision<1> {}
impl I16Compatible for Precision<2> {}
impl I16Compatible for Precision<3> {}
impl I16Compatible for Precision<4> {}

pub trait I32Compatible {}
impl I32Compatible for Precision<1> {}
impl I32Compatible for Precision<2> {}
impl I32Compatible for Precision<3> {}
impl I32Compatible for Precision<4> {}
impl I32Compatible for Precision<5> {}
impl I32Compatible for Precision<6> {}
impl I32Compatible for Precision<7> {}
impl I32Compatible for Precision<8> {}
impl I32Compatible for Precision<9> {}

pub trait I64Compatible {}

pub trait I128Compatible {}
impl I128Compatible for Precision<1> {}
impl I128Compatible for Precision<2> {}
impl I128Compatible for Precision<3> {}
impl I128Compatible for Precision<4> {}
impl I128Compatible for Precision<5> {}
impl I128Compatible for Precision<6> {}
impl I128Compatible for Precision<7> {}
impl I128Compatible for Precision<8> {}
impl I128Compatible for Precision<9> {}
impl I128Compatible for Precision<10> {}
impl I128Compatible for Precision<11> {}
impl I128Compatible for Precision<12> {}
impl I128Compatible for Precision<13> {}
impl I128Compatible for Precision<14> {}
impl I128Compatible for Precision<15> {}
impl I128Compatible for Precision<16> {}
impl I128Compatible for Precision<17> {}
impl I128Compatible for Precision<18> {}
impl I128Compatible for Precision<19> {}
impl I128Compatible for Precision<20> {}
impl I128Compatible for Precision<21> {}
impl I128Compatible for Precision<22> {}
impl I128Compatible for Precision<23> {}
impl I128Compatible for Precision<24> {}
impl I128Compatible for Precision<25> {}
impl I128Compatible for Precision<26> {}
impl I128Compatible for Precision<27> {}
impl I128Compatible for Precision<28> {}
impl I128Compatible for Precision<29> {}
impl I128Compatible for Precision<30> {}