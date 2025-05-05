::boiler::bundle!("src/math/q");
::boiler::expose!(
    add,
    constructor,
    engine,
    eq,
    mul,
    variant_generic,
    variant
);

use crate::common::int;
use crate::common::util;
use crate::math::engine::default_engine;
use ::core::ops;
use ::core::cmp;
use ::thiserror as error;
use ::num_traits as num;

use util::Util as _;
use num::ToPrimitive as _;
use int::Introspection as _;

pub type Result<T> = ::core::result::Result<T, Error>;
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(error::Error)]
pub enum Error {
    #[error("")]
    Overflow,
    #[error("")]
    Underflow,
    #[error("")]
    DivByZero,
    #[error("")]
    RemByZero,
    #[error("")]
    UnsupportedConversion,
    #[error("")]
    NegSqrt
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Q<const A: u8, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    _v: B,
    _engine: C
}

pub fn new<const A: u8, B>(v: B) -> Q<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int::Introspection {
    Q {
        _v: v,
        _engine: default_engine::new()
    }
}

pub fn new_with_custom_engine<const A: u8, B, C>(v: B, engine: C) -> Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    Q {
        _v: v,
        _engine: engine
    }
}

impl<const A: u8, B, C> Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    
    pub fn sqrt(&self) -> Result<Self> {
        self._engine.sqrt(self)
    }
}

impl<const A: u8, B, C> ops::Rem for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    B: num::CheckedRem,
    C: Engine {
    type Output = Result<Self>;

    fn rem(self, rhs: Self) -> <Self as ops::Rem>::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.rem(x, y)
    }
}

impl<const A: u8, B, C> ops::Mul for Q<A, B, C> 
where 
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.mul(x, y)
    }
}

impl<const A: u8, B, C> ops::Div for Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {
    type Output = Result<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        let x: &Self = &self;
        let y: &Self = &rhs;
        self._engine.div(x, y)
    }
}

impl<const A: u8, B, C> Ord for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {

    fn clamp(self, min: Self, max: Self) -> Self {
        if self > max {
            return max;
        }
        if self < min {
            return min;
        }
        self
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            return self;
        }
        other
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            return self;
        }
        other
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self > other {
            return cmp::Ordering::Greater
        }
        if self < other {
            return cmp::Ordering::Less
        }
        cmp::Ordering::Equal
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl<const A: u8, B, C> PartialOrd for Q<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {

    fn ge(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v >= y._v
    }

    fn le(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v <= y._v
    }

    fn gt(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v > y._v
    }

    fn lt(&self, other: &Self) -> bool {
        let x: &Self = self;
        let y: &Self = other;
        x._v < y._v
    }

    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.cmp(other).into_some()
    }
}

impl<const A: u8, B, C> PartialEq for Q<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: Engine {

    fn eq(&self, other: &Self) -> bool {
        let x: &Self = self;
        let x: &B = &x._v;
        let y: &Self = other;
        let y: &B = &y._v;
        x == y
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    use crate::common::int;
    use ::rstest::rstest;
    use ::core::fmt;

    #[rstest]
    #[case(new(5), new(50), new::<23, i32>(55))]
    #[case(new(1000), new(500), new::<12, i64>(1500))]
    #[case(new(-10), new(5), new::<9, i128>(-5))]
    #[case(new(35), new(15), new::<13, i8>(50))]
    #[case(new(250), new(300), new::<28, i16>(550))]
    #[case(new(400), new(100), new::<8, i32>(500))]
    #[case(new(-20), new(20), new::<5, i64>(0))]
    #[case(new(150), new(150), new::<18, i128>(300))]
    #[case(new(3), new(7), new::<2, i8>(10))]
    #[case(new(60), new(40), new::<20, i16>(100))]
    #[case(new(1000), new(-500), new::<17, i32>(500))]
    #[case(new(500), new(200), new::<25, i64>(700))]
    #[case(new(-100), new(50), new::<11, i128>(-50))]
    #[case(new(25), new(30), new::<30, i8>(55))]
    #[case(new(15), new(25), new::<32, i16>(40))]
    #[case(new(8), new(12), new::<21, i32>(20))]
    #[case(new(-25), new(50), new::<9, i64>(25))]
    #[case(new(100), new(-50), new::<12, i128>(50))]
    #[case(new(300), new(200), new::<16, i16>(500))]
    #[case(new(10), new(10), new::<19, i32>(20))]
    #[case(new(-50), new(-50), new::<22, i64>(-100))]
    #[case(new(250), new(100), new::<8, i128>(350))]
    #[case(new(75), new(25), new::<6, i8>(100))]
    #[case(new(150), new(250), new::<5, i16>(400))]
    #[case(new(300), new(100), new::<18, i32>(400))]
    #[case(new(-50), new(75), new::<9, i64>(25))]
    #[case(new(600), new(-300), new::<13, i128>(300))]
    #[case(new(45), new(55), new::<28, i8>(100))]
    #[case(new(30), new(70), new::<2, i16>(100))]
    #[case(new(10), new(200), new::<12, i32>(210))]
    #[case(new(-30), new(-20), new::<11, i64>(-50))]
    #[case(new(10), new(5), new::<30, i128>(15))]
    #[case(new(60), new(40), new::<8, i8>(100))]
    #[case(new(50), new(150), new::<22, i16>(200))]
    #[case(new(200), new(100), new::<20, i32>(300))]
    #[case(new(100), new(-100), new::<18, i64>(0))]
    #[case(new(300), new(100), new::<13, i128>(400))]
    #[case(new(-60), new(10), new::<17, i8>(-50))]
    #[case(new(500), new(100), new::<9, i16>(600))]
    #[case(new(200), new(300), new::<30, i32>(500))]
    #[case(new(10), new(10), new::<15, i64>(20))]
    #[case(new(-100), new(100), new::<8, i128>(0))]
    fn _test_add<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>, 
        #[case] y: Q<A, B, default_engine::DefaultEngine>, 
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>) 
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x + y).expect("");
        assert_eq!(result, correct_result);
    }

    #[rstest]
    #[case(new(10), new(10), new::<1, u8>(0))]
    #[case(new(10), new(10), new::<2, u8>(0))]
    #[case(new(10), new(10), new::<3, u8>(0))]
    #[case(new(10), new(10), new::<4, u8>(0))]
    #[case(new(10), new(10), new::<5, u8>(0))]
    #[case(new(10), new(10), new::<6, u8>(0))]
    #[case(new(10), new(10), new::<7, u8>(0))]
    #[case(new(10), new(10), new::<8, u8>(0))]
    #[case(new(10), new(10), new::<9, u8>(0))]
    #[case(new(10), new(10), new::<10, u8>(0))]
    #[case(new(10), new(10), new::<11, u8>(0))]
    #[case(new(10), new(10), new::<12, u8>(0))]
    #[case(new(10), new(10), new::<13, u8>(0))]
    #[case(new(10), new(10), new::<14, u8>(0))]
    #[case(new(10), new(10), new::<15, u8>(0))]
    #[case(new(10), new(10), new::<16, u8>(0))]
    #[case(new(10), new(10), new::<17, u8>(0))]
    #[case(new(10), new(10), new::<18, u8>(0))]
    #[case(new(10), new(10), new::<19, u8>(0))]
    #[case(new(10), new(10), new::<0, u8>(0))]
    #[case(new(10), new(10), new::<21, u8>(0))]
    #[case(new(10), new(10), new::<22, u8>(0))]
    #[case(new(10), new(10), new::<23, u8>(0))]
    #[case(new(10), new(10), new::<24, u8>(0))]
    #[case(new(10), new(10), new::<25, u8>(0))]
    #[case(new(10), new(10), new::<26, u8>(0))]
    #[case(new(10), new(10), new::<27, u8>(0))]
    #[case(new(10), new(10), new::<28, u8>(0))]
    #[case(new(10), new(10), new::<29, u8>(0))]
    #[case(new(10), new(10), new::<30, u8>(0))]
    #[case(new(10), new(10), new::<31, u8>(0))]
    #[case(new(10), new(10), new::<32, u8>(0))]
    #[case(new(10), new(10), new::<33, u8>(0))]
    #[case(new(10), new(10), new::<34, u8>(0))]
    #[case(new(10), new(10), new::<35, u8>(0))]
    #[case(new(10), new(10), new::<36, u8>(0))]
    #[case(new(10), new(10), new::<37, u8>(0))]
    #[case(new(10), new(10), new::<38, u8>(0))]
    #[case(new(10), new(10), new::<1, u16>(0))]
    #[case(new(10), new(10), new::<2, u16>(0))]
    #[case(new(10), new(10), new::<3, u16>(0))]
    #[case(new(10), new(10), new::<4, u16>(0))]
    #[case(new(10), new(10), new::<5, u16>(0))]
    #[case(new(10), new(10), new::<6, u16>(0))]
    #[case(new(10), new(10), new::<7, u16>(0))]
    #[case(new(10), new(10), new::<8, u16>(0))]
    #[case(new(10), new(10), new::<9, u16>(0))]
    #[case(new(10), new(10), new::<10, u16>(0))]
    #[case(new(10), new(10), new::<11, u16>(0))]
    #[case(new(10), new(10), new::<12, u16>(0))]
    #[case(new(10), new(10), new::<13, u16>(0))]
    #[case(new(10), new(10), new::<14, u16>(0))]
    #[case(new(10), new(10), new::<15, u16>(0))]
    #[case(new(10), new(10), new::<16, u16>(0))]
    #[case(new(10), new(10), new::<17, u16>(0))]
    #[case(new(10), new(10), new::<18, u16>(0))]
    #[case(new(10), new(10), new::<19, u16>(0))]
    #[case(new(10), new(10), new::<0, u16>(0))]
    #[case(new(10), new(10), new::<21, u16>(0))]
    #[case(new(10), new(10), new::<22, u16>(0))]
    #[case(new(10), new(10), new::<23, u16>(0))]
    #[case(new(10), new(10), new::<24, u16>(0))]
    #[case(new(10), new(10), new::<25, u16>(0))]
    #[case(new(10), new(10), new::<26, u16>(0))]
    #[case(new(10), new(10), new::<27, u16>(0))]
    #[case(new(10), new(10), new::<28, u16>(0))]
    #[case(new(10), new(10), new::<29, u16>(0))]
    #[case(new(10), new(10), new::<30, u16>(0))]
    #[case(new(10), new(10), new::<31, u16>(0))]
    #[case(new(10), new(10), new::<32, u16>(0))]
    #[case(new(10), new(10), new::<33, u16>(0))]
    #[case(new(10), new(10), new::<34, u16>(0))]
    #[case(new(10), new(10), new::<35, u16>(0))]
    #[case(new(10), new(10), new::<36, u16>(0))]
    #[case(new(10), new(10), new::<37, u16>(0))]
    #[case(new(10), new(10), new::<38, u16>(0))]
    #[case(new(10), new(10), new::<1, u32>(0))]
    #[case(new(10), new(10), new::<2, u32>(0))]
    #[case(new(10), new(10), new::<3, u32>(0))]
    #[case(new(10), new(10), new::<4, u32>(0))]
    #[case(new(10), new(10), new::<5, u32>(0))]
    #[case(new(10), new(10), new::<6, u32>(0))]
    #[case(new(10), new(10), new::<7, u32>(0))]
    #[case(new(10), new(10), new::<8, u32>(0))]
    #[case(new(10), new(10), new::<9, u32>(0))]
    #[case(new(10), new(10), new::<10, u32>(0))]
    #[case(new(10), new(10), new::<11, u32>(0))]
    #[case(new(10), new(10), new::<12, u32>(0))]
    #[case(new(10), new(10), new::<13, u32>(0))]
    #[case(new(10), new(10), new::<14, u32>(0))]
    #[case(new(10), new(10), new::<15, u32>(0))]
    #[case(new(10), new(10), new::<16, u32>(0))]
    #[case(new(10), new(10), new::<17, u32>(0))]
    #[case(new(10), new(10), new::<18, u32>(0))]
    #[case(new(10), new(10), new::<19, u32>(0))]
    #[case(new(10), new(10), new::<0, u32>(0))]
    #[case(new(10), new(10), new::<21, u32>(0))]
    #[case(new(10), new(10), new::<22, u32>(0))]
    #[case(new(10), new(10), new::<23, u32>(0))]
    #[case(new(10), new(10), new::<24, u32>(0))]
    #[case(new(10), new(10), new::<25, u32>(0))]
    #[case(new(10), new(10), new::<26, u32>(0))]
    #[case(new(10), new(10), new::<27, u32>(0))]
    #[case(new(10), new(10), new::<28, u32>(0))]
    #[case(new(10), new(10), new::<29, u32>(0))]
    #[case(new(10), new(10), new::<30, u32>(0))]
    #[case(new(10), new(10), new::<31, u32>(0))]
    #[case(new(10), new(10), new::<32, u32>(0))]
    #[case(new(10), new(10), new::<33, u32>(0))]
    #[case(new(10), new(10), new::<34, u32>(0))]
    #[case(new(10), new(10), new::<35, u32>(0))]
    #[case(new(10), new(10), new::<36, u32>(0))]
    #[case(new(10), new(10), new::<37, u32>(0))]
    #[case(new(10), new(10), new::<38, u32>(0))]
    #[case(new(10), new(10), new::<1, u64>(0))]
    #[case(new(10), new(10), new::<2, u64>(0))]
    #[case(new(10), new(10), new::<3, u64>(0))]
    #[case(new(10), new(10), new::<4, u64>(0))]
    #[case(new(10), new(10), new::<5, u64>(0))]
    #[case(new(10), new(10), new::<6, u64>(0))]
    #[case(new(10), new(10), new::<7, u64>(0))]
    #[case(new(10), new(10), new::<8, u64>(0))]
    #[case(new(10), new(10), new::<9, u64>(0))]
    #[case(new(10), new(10), new::<10, u64>(0))]
    #[case(new(10), new(10), new::<11, u64>(0))]
    #[case(new(10), new(10), new::<12, u64>(0))]
    #[case(new(10), new(10), new::<13, u64>(0))]
    #[case(new(10), new(10), new::<14, u64>(0))]
    #[case(new(10), new(10), new::<15, u64>(0))]
    #[case(new(10), new(10), new::<16, u64>(0))]
    #[case(new(10), new(10), new::<17, u64>(0))]
    #[case(new(10), new(10), new::<18, u64>(0))]
    #[case(new(10), new(10), new::<19, u64>(0))]
    #[case(new(10), new(10), new::<0, u64>(0))]
    #[case(new(10), new(10), new::<21, u64>(0))]
    #[case(new(10), new(10), new::<22, u64>(0))]
    #[case(new(10), new(10), new::<23, u64>(0))]
    #[case(new(10), new(10), new::<24, u64>(0))]
    #[case(new(10), new(10), new::<25, u64>(0))]
    #[case(new(10), new(10), new::<26, u64>(0))]
    #[case(new(10), new(10), new::<27, u64>(0))]
    #[case(new(10), new(10), new::<28, u64>(0))]
    #[case(new(10), new(10), new::<29, u64>(0))]
    #[case(new(10), new(10), new::<30, u64>(0))]
    #[case(new(10), new(10), new::<31, u64>(0))]
    #[case(new(10), new(10), new::<32, u64>(0))]
    #[case(new(10), new(10), new::<33, u64>(0))]
    #[case(new(10), new(10), new::<34, u64>(0))]
    #[case(new(10), new(10), new::<35, u64>(0))]
    #[case(new(10), new(10), new::<36, u64>(0))]
    #[case(new(10), new(10), new::<37, u64>(0))]
    #[case(new(10), new(10), new::<38, u64>(0))]
    #[case(new(10), new(10), new::<1, u128>(0))]
    #[case(new(10), new(10), new::<2, u128>(0))]
    #[case(new(10), new(10), new::<3, u128>(0))]
    #[case(new(10), new(10), new::<4, u128>(0))]
    #[case(new(10), new(10), new::<5, u128>(0))]
    #[case(new(10), new(10), new::<6, u128>(0))]
    #[case(new(10), new(10), new::<7, u128>(0))]
    #[case(new(10), new(10), new::<8, u128>(0))]
    #[case(new(10), new(10), new::<9, u128>(0))]
    #[case(new(10), new(10), new::<10, u128>(0))]
    #[case(new(10), new(10), new::<11, u128>(0))]
    #[case(new(10), new(10), new::<12, u128>(0))]
    #[case(new(10), new(10), new::<13, u128>(0))]
    #[case(new(10), new(10), new::<14, u128>(0))]
    #[case(new(10), new(10), new::<15, u128>(0))]
    #[case(new(10), new(10), new::<16, u128>(0))]
    #[case(new(10), new(10), new::<17, u128>(0))]
    #[case(new(10), new(10), new::<18, u128>(0))]
    #[case(new(10), new(10), new::<19, u128>(0))]
    #[case(new(10), new(10), new::<0, u128>(0))]
    #[case(new(10), new(10), new::<21, u128>(0))]
    #[case(new(10), new(10), new::<22, u128>(0))]
    #[case(new(10), new(10), new::<23, u128>(0))]
    #[case(new(10), new(10), new::<24, u128>(0))]
    #[case(new(10), new(10), new::<25, u128>(0))]
    #[case(new(10), new(10), new::<26, u128>(0))]
    #[case(new(10), new(10), new::<27, u128>(0))]
    #[case(new(10), new(10), new::<28, u128>(0))]
    #[case(new(10), new(10), new::<29, u128>(0))]
    #[case(new(10), new(10), new::<30, u128>(0))]
    #[case(new(10), new(10), new::<31, u128>(0))]
    #[case(new(10), new(10), new::<32, u128>(0))]
    #[case(new(10), new(10), new::<33, u128>(0))]
    #[case(new(10), new(10), new::<34, u128>(0))]
    #[case(new(10), new(10), new::<35, u128>(0))]
    #[case(new(10), new(10), new::<36, u128>(0))]
    #[case(new(10), new(10), new::<37, u128>(0))]
    #[case(new(10), new(10), new::<38, u128>(0))]
    #[case(new(10), new(10), new::<1, i8>(0))]
    #[case(new(10), new(10), new::<2, i8>(0))]
    #[case(new(10), new(10), new::<3, i8>(0))]
    #[case(new(10), new(10), new::<4, i8>(0))]
    #[case(new(10), new(10), new::<5, i8>(0))]
    #[case(new(10), new(10), new::<6, i8>(0))]
    #[case(new(10), new(10), new::<7, i8>(0))]
    #[case(new(10), new(10), new::<8, i8>(0))]
    #[case(new(10), new(10), new::<9, i8>(0))]
    #[case(new(10), new(10), new::<10, i8>(0))]
    #[case(new(10), new(10), new::<11, i8>(0))]
    #[case(new(10), new(10), new::<12, i8>(0))]
    #[case(new(10), new(10), new::<13, i8>(0))]
    #[case(new(10), new(10), new::<14, i8>(0))]
    #[case(new(10), new(10), new::<15, i8>(0))]
    #[case(new(10), new(10), new::<16, i8>(0))]
    #[case(new(10), new(10), new::<17, i8>(0))]
    #[case(new(10), new(10), new::<18, i8>(0))]
    #[case(new(10), new(10), new::<19, i8>(0))]
    #[case(new(10), new(10), new::<0, i8>(0))]
    #[case(new(10), new(10), new::<21, i8>(0))]
    #[case(new(10), new(10), new::<22, i8>(0))]
    #[case(new(10), new(10), new::<23, i8>(0))]
    #[case(new(10), new(10), new::<24, i8>(0))]
    #[case(new(10), new(10), new::<25, i8>(0))]
    #[case(new(10), new(10), new::<26, i8>(0))]
    #[case(new(10), new(10), new::<27, i8>(0))]
    #[case(new(10), new(10), new::<28, i8>(0))]
    #[case(new(10), new(10), new::<29, i8>(0))]
    #[case(new(10), new(10), new::<30, i8>(0))]
    #[case(new(10), new(10), new::<31, i8>(0))]
    #[case(new(10), new(10), new::<32, i8>(0))]
    #[case(new(10), new(10), new::<33, i8>(0))]
    #[case(new(10), new(10), new::<34, i8>(0))]
    #[case(new(10), new(10), new::<35, i8>(0))]
    #[case(new(10), new(10), new::<36, i8>(0))]
    #[case(new(10), new(10), new::<37, i8>(0))]
    #[case(new(10), new(10), new::<38, i8>(0))]
    #[case(new(10), new(10), new::<1, i16>(0))]
    #[case(new(10), new(10), new::<2, i16>(0))]
    #[case(new(10), new(10), new::<3, i16>(0))]
    #[case(new(10), new(10), new::<4, i16>(0))]
    #[case(new(10), new(10), new::<5, i16>(0))]
    #[case(new(10), new(10), new::<6, i16>(0))]
    #[case(new(10), new(10), new::<7, i16>(0))]
    #[case(new(10), new(10), new::<8, i16>(0))]
    #[case(new(10), new(10), new::<9, i16>(0))]
    #[case(new(10), new(10), new::<10, i16>(0))]
    #[case(new(10), new(10), new::<11, i16>(0))]
    #[case(new(10), new(10), new::<12, i16>(0))]
    #[case(new(10), new(10), new::<13, i16>(0))]
    #[case(new(10), new(10), new::<14, i16>(0))]
    #[case(new(10), new(10), new::<15, i16>(0))]
    #[case(new(10), new(10), new::<16, i16>(0))]
    #[case(new(10), new(10), new::<17, i16>(0))]
    #[case(new(10), new(10), new::<18, i16>(0))]
    #[case(new(10), new(10), new::<19, i16>(0))]
    #[case(new(10), new(10), new::<0, i16>(0))]
    #[case(new(10), new(10), new::<21, i16>(0))]
    #[case(new(10), new(10), new::<22, i16>(0))]
    #[case(new(10), new(10), new::<23, i16>(0))]
    #[case(new(10), new(10), new::<24, i16>(0))]
    #[case(new(10), new(10), new::<25, i16>(0))]
    #[case(new(10), new(10), new::<26, i16>(0))]
    #[case(new(10), new(10), new::<27, i16>(0))]
    #[case(new(10), new(10), new::<28, i16>(0))]
    #[case(new(10), new(10), new::<29, i16>(0))]
    #[case(new(10), new(10), new::<30, i16>(0))]
    #[case(new(10), new(10), new::<31, i16>(0))]
    #[case(new(10), new(10), new::<32, i16>(0))]
    #[case(new(10), new(10), new::<33, i16>(0))]
    #[case(new(10), new(10), new::<34, i16>(0))]
    #[case(new(10), new(10), new::<35, i16>(0))]
    #[case(new(10), new(10), new::<36, i16>(0))]
    #[case(new(10), new(10), new::<37, i16>(0))]
    #[case(new(10), new(10), new::<38, i16>(0))]
    #[case(new(10), new(10), new::<1, i32>(0))]
    #[case(new(10), new(10), new::<2, i32>(0))]
    #[case(new(10), new(10), new::<3, i32>(0))]
    #[case(new(10), new(10), new::<4, i32>(0))]
    #[case(new(10), new(10), new::<5, i32>(0))]
    #[case(new(10), new(10), new::<6, i32>(0))]
    #[case(new(10), new(10), new::<7, i32>(0))]
    #[case(new(10), new(10), new::<8, i32>(0))]
    #[case(new(10), new(10), new::<9, i32>(0))]
    #[case(new(10), new(10), new::<10, i32>(0))]
    #[case(new(10), new(10), new::<11, i32>(0))]
    #[case(new(10), new(10), new::<12, i32>(0))]
    #[case(new(10), new(10), new::<13, i32>(0))]
    #[case(new(10), new(10), new::<14, i32>(0))]
    #[case(new(10), new(10), new::<15, i32>(0))]
    #[case(new(10), new(10), new::<16, i32>(0))]
    #[case(new(10), new(10), new::<17, i32>(0))]
    #[case(new(10), new(10), new::<18, i32>(0))]
    #[case(new(10), new(10), new::<19, i32>(0))]
    #[case(new(10), new(10), new::<0, i32>(0))]
    #[case(new(10), new(10), new::<21, i32>(0))]
    #[case(new(10), new(10), new::<22, i32>(0))]
    #[case(new(10), new(10), new::<23, i32>(0))]
    #[case(new(10), new(10), new::<24, i32>(0))]
    #[case(new(10), new(10), new::<25, i32>(0))]
    #[case(new(10), new(10), new::<26, i32>(0))]
    #[case(new(10), new(10), new::<27, i32>(0))]
    #[case(new(10), new(10), new::<28, i32>(0))]
    #[case(new(10), new(10), new::<29, i32>(0))]
    #[case(new(10), new(10), new::<30, i32>(0))]
    #[case(new(10), new(10), new::<31, i32>(0))]
    #[case(new(10), new(10), new::<32, i32>(0))]
    #[case(new(10), new(10), new::<33, i32>(0))]
    #[case(new(10), new(10), new::<34, i32>(0))]
    #[case(new(10), new(10), new::<35, i32>(0))]
    #[case(new(10), new(10), new::<36, i32>(0))]
    #[case(new(10), new(10), new::<37, i32>(0))]
    #[case(new(10), new(10), new::<38, i32>(0))]
    #[case(new(10), new(10), new::<1, i64>(0))]
    #[case(new(10), new(10), new::<2, i64>(0))]
    #[case(new(10), new(10), new::<3, i64>(0))]
    #[case(new(10), new(10), new::<4, i64>(0))]
    #[case(new(10), new(10), new::<5, i64>(0))]
    #[case(new(10), new(10), new::<6, i64>(0))]
    #[case(new(10), new(10), new::<7, i64>(0))]
    #[case(new(10), new(10), new::<8, i64>(0))]
    #[case(new(10), new(10), new::<9, i64>(0))]
    #[case(new(10), new(10), new::<10, i64>(0))]
    #[case(new(10), new(10), new::<11, i64>(0))]
    #[case(new(10), new(10), new::<12, i64>(0))]
    #[case(new(10), new(10), new::<13, i64>(0))]
    #[case(new(10), new(10), new::<14, i64>(0))]
    #[case(new(10), new(10), new::<15, i64>(0))]
    #[case(new(10), new(10), new::<16, i64>(0))]
    #[case(new(10), new(10), new::<17, i64>(0))]
    #[case(new(10), new(10), new::<18, i64>(0))]
    #[case(new(10), new(10), new::<19, i64>(0))]
    #[case(new(10), new(10), new::<0, i64>(0))]
    #[case(new(10), new(10), new::<21, i64>(0))]
    #[case(new(10), new(10), new::<22, i64>(0))]
    #[case(new(10), new(10), new::<23, i64>(0))]
    #[case(new(10), new(10), new::<24, i64>(0))]
    #[case(new(10), new(10), new::<25, i64>(0))]
    #[case(new(10), new(10), new::<26, i64>(0))]
    #[case(new(10), new(10), new::<27, i64>(0))]
    #[case(new(10), new(10), new::<28, i64>(0))]
    #[case(new(10), new(10), new::<29, i64>(0))]
    #[case(new(10), new(10), new::<30, i64>(0))]
    #[case(new(10), new(10), new::<31, i64>(0))]
    #[case(new(10), new(10), new::<32, i64>(0))]
    #[case(new(10), new(10), new::<33, i64>(0))]
    #[case(new(10), new(10), new::<34, i64>(0))]
    #[case(new(10), new(10), new::<35, i64>(0))]
    #[case(new(10), new(10), new::<36, i64>(0))]
    #[case(new(10), new(10), new::<37, i64>(0))]
    #[case(new(10), new(10), new::<38, i64>(0))]
    #[case(new(10), new(10), new::<1, i128>(0))]
    #[case(new(10), new(10), new::<2, i128>(0))]
    #[case(new(10), new(10), new::<3, i128>(0))]
    #[case(new(10), new(10), new::<4, i128>(0))]
    #[case(new(10), new(10), new::<5, i128>(0))]
    #[case(new(10), new(10), new::<6, i128>(0))]
    #[case(new(10), new(10), new::<7, i128>(0))]
    #[case(new(10), new(10), new::<8, i128>(0))]
    #[case(new(10), new(10), new::<9, i128>(0))]
    #[case(new(10), new(10), new::<10, i128>(0))]
    #[case(new(10), new(10), new::<11, i128>(0))]
    #[case(new(10), new(10), new::<12, i128>(0))]
    #[case(new(10), new(10), new::<13, i128>(0))]
    #[case(new(10), new(10), new::<14, i128>(0))]
    #[case(new(10), new(10), new::<15, i128>(0))]
    #[case(new(10), new(10), new::<16, i128>(0))]
    #[case(new(10), new(10), new::<17, i128>(0))]
    #[case(new(10), new(10), new::<18, i128>(0))]
    #[case(new(10), new(10), new::<19, i128>(0))]
    #[case(new(10), new(10), new::<0, i128>(0))]
    #[case(new(10), new(10), new::<21, i128>(0))]
    #[case(new(10), new(10), new::<22, i128>(0))]
    #[case(new(10), new(10), new::<23, i128>(0))]
    #[case(new(10), new(10), new::<24, i128>(0))]
    #[case(new(10), new(10), new::<25, i128>(0))]
    #[case(new(10), new(10), new::<26, i128>(0))]
    #[case(new(10), new(10), new::<27, i128>(0))]
    #[case(new(10), new(10), new::<28, i128>(0))]
    #[case(new(10), new(10), new::<29, i128>(0))]
    #[case(new(10), new(10), new::<30, i128>(0))]
    #[case(new(10), new(10), new::<31, i128>(0))]
    #[case(new(10), new(10), new::<32, i128>(0))]
    #[case(new(10), new(10), new::<33, i128>(0))]
    #[case(new(10), new(10), new::<34, i128>(0))]
    #[case(new(10), new(10), new::<35, i128>(0))]
    #[case(new(10), new(10), new::<36, i128>(0))]
    #[case(new(10), new(10), new::<37, i128>(0))]
    #[case(new(10), new(10), new::<38, i128>(0))]
    fn _test_sub<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>,
        #[case] y: Q<A, B, default_engine::DefaultEngine>,
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>)
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x - y).unwrap();
        assert_eq!(result, correct_result);
    }

    #[rstest]
    #[case(new(10), new(10), new::<1, u8>(20))]
    #[case(new(10), new(10), new::<2, u8>(20))]
    #[case(new(10), new(10), new::<3, u8>(20))]
    #[case(new(10), new(10), new::<4, u8>(20))]
    #[case(new(10), new(10), new::<5, u8>(20))]
    #[case(new(10), new(10), new::<6, u8>(20))]
    #[case(new(10), new(10), new::<7, u8>(20))]
    #[case(new(10), new(10), new::<8, u8>(20))]
    #[case(new(10), new(10), new::<9, u8>(20))]
    #[case(new(10), new(10), new::<10, u8>(20))]
    #[case(new(10), new(10), new::<11, u8>(20))]
    #[case(new(10), new(10), new::<12, u8>(20))]
    #[case(new(10), new(10), new::<13, u8>(20))]
    #[case(new(10), new(10), new::<14, u8>(20))]
    #[case(new(10), new(10), new::<15, u8>(20))]
    #[case(new(10), new(10), new::<16, u8>(20))]
    #[case(new(10), new(10), new::<17, u8>(20))]
    #[case(new(10), new(10), new::<18, u8>(20))]
    #[case(new(10), new(10), new::<19, u8>(20))]
    #[case(new(10), new(10), new::<20, u8>(20))]
    #[case(new(10), new(10), new::<21, u8>(20))]
    #[case(new(10), new(10), new::<22, u8>(20))]
    #[case(new(10), new(10), new::<23, u8>(20))]
    #[case(new(10), new(10), new::<24, u8>(20))]
    #[case(new(10), new(10), new::<25, u8>(20))]
    #[case(new(10), new(10), new::<26, u8>(20))]
    #[case(new(10), new(10), new::<27, u8>(20))]
    #[case(new(10), new(10), new::<28, u8>(20))]
    #[case(new(10), new(10), new::<29, u8>(20))]
    #[case(new(10), new(10), new::<30, u8>(20))]
    #[case(new(10), new(10), new::<31, u8>(20))]
    #[case(new(10), new(10), new::<32, u8>(20))]
    #[case(new(10), new(10), new::<33, u8>(20))]
    #[case(new(10), new(10), new::<34, u8>(20))]
    #[case(new(10), new(10), new::<35, u8>(20))]
    #[case(new(10), new(10), new::<36, u8>(20))]
    #[case(new(10), new(10), new::<37, u8>(20))]
    #[case(new(10), new(10), new::<38, u8>(20))]
    #[case(new(10), new(10), new::<1, u16>(20))]
    #[case(new(10), new(10), new::<2, u16>(20))]
    #[case(new(10), new(10), new::<3, u16>(20))]
    #[case(new(10), new(10), new::<4, u16>(20))]
    #[case(new(10), new(10), new::<5, u16>(20))]
    #[case(new(10), new(10), new::<6, u16>(20))]
    #[case(new(10), new(10), new::<7, u16>(20))]
    #[case(new(10), new(10), new::<8, u16>(20))]
    #[case(new(10), new(10), new::<9, u16>(20))]
    #[case(new(10), new(10), new::<10, u16>(20))]
    #[case(new(10), new(10), new::<11, u16>(20))]
    #[case(new(10), new(10), new::<12, u16>(20))]
    #[case(new(10), new(10), new::<13, u16>(20))]
    #[case(new(10), new(10), new::<14, u16>(20))]
    #[case(new(10), new(10), new::<15, u16>(20))]
    #[case(new(10), new(10), new::<16, u16>(20))]
    #[case(new(10), new(10), new::<17, u16>(20))]
    #[case(new(10), new(10), new::<18, u16>(20))]
    #[case(new(10), new(10), new::<19, u16>(20))]
    #[case(new(10), new(10), new::<20, u16>(20))]
    #[case(new(10), new(10), new::<21, u16>(20))]
    #[case(new(10), new(10), new::<22, u16>(20))]
    #[case(new(10), new(10), new::<23, u16>(20))]
    #[case(new(10), new(10), new::<24, u16>(20))]
    #[case(new(10), new(10), new::<25, u16>(20))]
    #[case(new(10), new(10), new::<26, u16>(20))]
    #[case(new(10), new(10), new::<27, u16>(20))]
    #[case(new(10), new(10), new::<28, u16>(20))]
    #[case(new(10), new(10), new::<29, u16>(20))]
    #[case(new(10), new(10), new::<30, u16>(20))]
    #[case(new(10), new(10), new::<31, u16>(20))]
    #[case(new(10), new(10), new::<32, u16>(20))]
    #[case(new(10), new(10), new::<33, u16>(20))]
    #[case(new(10), new(10), new::<34, u16>(20))]
    #[case(new(10), new(10), new::<35, u16>(20))]
    #[case(new(10), new(10), new::<36, u16>(20))]
    #[case(new(10), new(10), new::<37, u16>(20))]
    #[case(new(10), new(10), new::<38, u16>(20))]
    #[case(new(10), new(10), new::<1, u32>(20))]
    #[case(new(10), new(10), new::<2, u32>(20))]
    #[case(new(10), new(10), new::<3, u32>(20))]
    #[case(new(10), new(10), new::<4, u32>(20))]
    #[case(new(10), new(10), new::<5, u32>(20))]
    #[case(new(10), new(10), new::<6, u32>(20))]
    #[case(new(10), new(10), new::<7, u32>(20))]
    #[case(new(10), new(10), new::<8, u32>(20))]
    #[case(new(10), new(10), new::<9, u32>(20))]
    #[case(new(10), new(10), new::<10, u32>(20))]
    #[case(new(10), new(10), new::<11, u32>(20))]
    #[case(new(10), new(10), new::<12, u32>(20))]
    #[case(new(10), new(10), new::<13, u32>(20))]
    #[case(new(10), new(10), new::<14, u32>(20))]
    #[case(new(10), new(10), new::<15, u32>(20))]
    #[case(new(10), new(10), new::<16, u32>(20))]
    #[case(new(10), new(10), new::<17, u32>(20))]
    #[case(new(10), new(10), new::<18, u32>(20))]
    #[case(new(10), new(10), new::<19, u32>(20))]
    #[case(new(10), new(10), new::<20, u32>(20))]
    #[case(new(10), new(10), new::<21, u32>(20))]
    #[case(new(10), new(10), new::<22, u32>(20))]
    #[case(new(10), new(10), new::<23, u32>(20))]
    #[case(new(10), new(10), new::<24, u32>(20))]
    #[case(new(10), new(10), new::<25, u32>(20))]
    #[case(new(10), new(10), new::<26, u32>(20))]
    #[case(new(10), new(10), new::<27, u32>(20))]
    #[case(new(10), new(10), new::<28, u32>(20))]
    #[case(new(10), new(10), new::<29, u32>(20))]
    #[case(new(10), new(10), new::<30, u32>(20))]
    #[case(new(10), new(10), new::<31, u32>(20))]
    #[case(new(10), new(10), new::<32, u32>(20))]
    #[case(new(10), new(10), new::<33, u32>(20))]
    #[case(new(10), new(10), new::<34, u32>(20))]
    #[case(new(10), new(10), new::<35, u32>(20))]
    #[case(new(10), new(10), new::<36, u32>(20))]
    #[case(new(10), new(10), new::<37, u32>(20))]
    #[case(new(10), new(10), new::<38, u32>(20))]
    #[case(new(10), new(10), new::<1, u64>(20))]
    #[case(new(10), new(10), new::<2, u64>(20))]
    #[case(new(10), new(10), new::<3, u64>(20))]
    #[case(new(10), new(10), new::<4, u64>(20))]
    #[case(new(10), new(10), new::<5, u64>(20))]
    #[case(new(10), new(10), new::<6, u64>(20))]
    #[case(new(10), new(10), new::<7, u64>(20))]
    #[case(new(10), new(10), new::<8, u64>(20))]
    #[case(new(10), new(10), new::<9, u64>(20))]
    #[case(new(10), new(10), new::<10, u64>(20))]
    #[case(new(10), new(10), new::<11, u64>(20))]
    #[case(new(10), new(10), new::<12, u64>(20))]
    #[case(new(10), new(10), new::<13, u64>(20))]
    #[case(new(10), new(10), new::<14, u64>(20))]
    #[case(new(10), new(10), new::<15, u64>(20))]
    #[case(new(10), new(10), new::<16, u64>(20))]
    #[case(new(10), new(10), new::<17, u64>(20))]
    #[case(new(10), new(10), new::<18, u64>(20))]
    #[case(new(10), new(10), new::<19, u64>(20))]
    #[case(new(10), new(10), new::<20, u64>(20))]
    #[case(new(10), new(10), new::<21, u64>(20))]
    #[case(new(10), new(10), new::<22, u64>(20))]
    #[case(new(10), new(10), new::<23, u64>(20))]
    #[case(new(10), new(10), new::<24, u64>(20))]
    #[case(new(10), new(10), new::<25, u64>(20))]
    #[case(new(10), new(10), new::<26, u64>(20))]
    #[case(new(10), new(10), new::<27, u64>(20))]
    #[case(new(10), new(10), new::<28, u64>(20))]
    #[case(new(10), new(10), new::<29, u64>(20))]
    #[case(new(10), new(10), new::<30, u64>(20))]
    #[case(new(10), new(10), new::<31, u64>(20))]
    #[case(new(10), new(10), new::<32, u64>(20))]
    #[case(new(10), new(10), new::<33, u64>(20))]
    #[case(new(10), new(10), new::<34, u64>(20))]
    #[case(new(10), new(10), new::<35, u64>(20))]
    #[case(new(10), new(10), new::<36, u64>(20))]
    #[case(new(10), new(10), new::<37, u64>(20))]
    #[case(new(10), new(10), new::<38, u64>(20))]
    #[case(new(10), new(10), new::<1, u128>(20))]
    #[case(new(10), new(10), new::<2, u128>(20))]
    #[case(new(10), new(10), new::<3, u128>(20))]
    #[case(new(10), new(10), new::<4, u128>(20))]
    #[case(new(10), new(10), new::<5, u128>(20))]
    #[case(new(10), new(10), new::<6, u128>(20))]
    #[case(new(10), new(10), new::<7, u128>(20))]
    #[case(new(10), new(10), new::<8, u128>(20))]
    #[case(new(10), new(10), new::<9, u128>(20))]
    #[case(new(10), new(10), new::<10, u128>(20))]
    #[case(new(10), new(10), new::<11, u128>(20))]
    #[case(new(10), new(10), new::<12, u128>(20))]
    #[case(new(10), new(10), new::<13, u128>(20))]
    #[case(new(10), new(10), new::<14, u128>(20))]
    #[case(new(10), new(10), new::<15, u128>(20))]
    #[case(new(10), new(10), new::<16, u128>(20))]
    #[case(new(10), new(10), new::<17, u128>(20))]
    #[case(new(10), new(10), new::<18, u128>(20))]
    #[case(new(10), new(10), new::<19, u128>(20))]
    #[case(new(10), new(10), new::<20, u128>(20))]
    #[case(new(10), new(10), new::<21, u128>(20))]
    #[case(new(10), new(10), new::<22, u128>(20))]
    #[case(new(10), new(10), new::<23, u128>(20))]
    #[case(new(10), new(10), new::<24, u128>(20))]
    #[case(new(10), new(10), new::<25, u128>(20))]
    #[case(new(10), new(10), new::<26, u128>(20))]
    #[case(new(10), new(10), new::<27, u128>(20))]
    #[case(new(10), new(10), new::<28, u128>(20))]
    #[case(new(10), new(10), new::<29, u128>(20))]
    #[case(new(10), new(10), new::<30, u128>(20))]
    #[case(new(10), new(10), new::<31, u128>(20))]
    #[case(new(10), new(10), new::<32, u128>(20))]
    #[case(new(10), new(10), new::<33, u128>(20))]
    #[case(new(10), new(10), new::<34, u128>(20))]
    #[case(new(10), new(10), new::<35, u128>(20))]
    #[case(new(10), new(10), new::<36, u128>(20))]
    #[case(new(10), new(10), new::<37, u128>(20))]
    #[case(new(10), new(10), new::<38, u128>(20))]
    #[case(new(10), new(10), new::<1, i8>(20))]
    #[case(new(10), new(10), new::<2, i8>(20))]
    #[case(new(10), new(10), new::<3, i8>(20))]
    #[case(new(10), new(10), new::<4, i8>(20))]
    #[case(new(10), new(10), new::<5, i8>(20))]
    #[case(new(10), new(10), new::<6, i8>(20))]
    #[case(new(10), new(10), new::<7, i8>(20))]
    #[case(new(10), new(10), new::<8, i8>(20))]
    #[case(new(10), new(10), new::<9, i8>(20))]
    #[case(new(10), new(10), new::<10, i8>(20))]
    #[case(new(10), new(10), new::<11, i8>(20))]
    #[case(new(10), new(10), new::<12, i8>(20))]
    #[case(new(10), new(10), new::<13, i8>(20))]
    #[case(new(10), new(10), new::<14, i8>(20))]
    #[case(new(10), new(10), new::<15, i8>(20))]
    #[case(new(10), new(10), new::<16, i8>(20))]
    #[case(new(10), new(10), new::<17, i8>(20))]
    #[case(new(10), new(10), new::<18, i8>(20))]
    #[case(new(10), new(10), new::<19, i8>(20))]
    #[case(new(10), new(10), new::<20, i8>(20))]
    #[case(new(10), new(10), new::<21, i8>(20))]
    #[case(new(10), new(10), new::<22, i8>(20))]
    #[case(new(10), new(10), new::<23, i8>(20))]
    #[case(new(10), new(10), new::<24, i8>(20))]
    #[case(new(10), new(10), new::<25, i8>(20))]
    #[case(new(10), new(10), new::<26, i8>(20))]
    #[case(new(10), new(10), new::<27, i8>(20))]
    #[case(new(10), new(10), new::<28, i8>(20))]
    #[case(new(10), new(10), new::<29, i8>(20))]
    #[case(new(10), new(10), new::<30, i8>(20))]
    #[case(new(10), new(10), new::<31, i8>(20))]
    #[case(new(10), new(10), new::<32, i8>(20))]
    #[case(new(10), new(10), new::<33, i8>(20))]
    #[case(new(10), new(10), new::<34, i8>(20))]
    #[case(new(10), new(10), new::<35, i8>(20))]
    #[case(new(10), new(10), new::<36, i8>(20))]
    #[case(new(10), new(10), new::<37, i8>(20))]
    #[case(new(10), new(10), new::<38, i8>(20))]
    #[case(new(10), new(10), new::<1, i16>(20))]
    #[case(new(10), new(10), new::<2, i16>(20))]
    #[case(new(10), new(10), new::<3, i16>(20))]
    #[case(new(10), new(10), new::<4, i16>(20))]
    #[case(new(10), new(10), new::<5, i16>(20))]
    #[case(new(10), new(10), new::<6, i16>(20))]
    #[case(new(10), new(10), new::<7, i16>(20))]
    #[case(new(10), new(10), new::<8, i16>(20))]
    #[case(new(10), new(10), new::<9, i16>(20))]
    #[case(new(10), new(10), new::<10, i16>(20))]
    #[case(new(10), new(10), new::<11, i16>(20))]
    #[case(new(10), new(10), new::<12, i16>(20))]
    #[case(new(10), new(10), new::<13, i16>(20))]
    #[case(new(10), new(10), new::<14, i16>(20))]
    #[case(new(10), new(10), new::<15, i16>(20))]
    #[case(new(10), new(10), new::<16, i16>(20))]
    #[case(new(10), new(10), new::<17, i16>(20))]
    #[case(new(10), new(10), new::<18, i16>(20))]
    #[case(new(10), new(10), new::<19, i16>(20))]
    #[case(new(10), new(10), new::<20, i16>(20))]
    #[case(new(10), new(10), new::<21, i16>(20))]
    #[case(new(10), new(10), new::<22, i16>(20))]
    #[case(new(10), new(10), new::<23, i16>(20))]
    #[case(new(10), new(10), new::<24, i16>(20))]
    #[case(new(10), new(10), new::<25, i16>(20))]
    #[case(new(10), new(10), new::<26, i16>(20))]
    #[case(new(10), new(10), new::<27, i16>(20))]
    #[case(new(10), new(10), new::<28, i16>(20))]
    #[case(new(10), new(10), new::<29, i16>(20))]
    #[case(new(10), new(10), new::<30, i16>(20))]
    #[case(new(10), new(10), new::<31, i16>(20))]
    #[case(new(10), new(10), new::<32, i16>(20))]
    #[case(new(10), new(10), new::<33, i16>(20))]
    #[case(new(10), new(10), new::<34, i16>(20))]
    #[case(new(10), new(10), new::<35, i16>(20))]
    #[case(new(10), new(10), new::<36, i16>(20))]
    #[case(new(10), new(10), new::<37, i16>(20))]
    #[case(new(10), new(10), new::<38, i16>(20))]
    #[case(new(10), new(10), new::<1, i32>(20))]
    #[case(new(10), new(10), new::<2, i32>(20))]
    #[case(new(10), new(10), new::<3, i32>(20))]
    #[case(new(10), new(10), new::<4, i32>(20))]
    #[case(new(10), new(10), new::<5, i32>(20))]
    #[case(new(10), new(10), new::<6, i32>(20))]
    #[case(new(10), new(10), new::<7, i32>(20))]
    #[case(new(10), new(10), new::<8, i32>(20))]
    #[case(new(10), new(10), new::<9, i32>(20))]
    #[case(new(10), new(10), new::<10, i32>(20))]
    #[case(new(10), new(10), new::<11, i32>(20))]
    #[case(new(10), new(10), new::<12, i32>(20))]
    #[case(new(10), new(10), new::<13, i32>(20))]
    #[case(new(10), new(10), new::<14, i32>(20))]
    #[case(new(10), new(10), new::<15, i32>(20))]
    #[case(new(10), new(10), new::<16, i32>(20))]
    #[case(new(10), new(10), new::<17, i32>(20))]
    #[case(new(10), new(10), new::<18, i32>(20))]
    #[case(new(10), new(10), new::<19, i32>(20))]
    #[case(new(10), new(10), new::<20, i32>(20))]
    #[case(new(10), new(10), new::<21, i32>(20))]
    #[case(new(10), new(10), new::<22, i32>(20))]
    #[case(new(10), new(10), new::<23, i32>(20))]
    #[case(new(10), new(10), new::<24, i32>(20))]
    #[case(new(10), new(10), new::<25, i32>(20))]
    #[case(new(10), new(10), new::<26, i32>(20))]
    #[case(new(10), new(10), new::<27, i32>(20))]
    #[case(new(10), new(10), new::<28, i32>(20))]
    #[case(new(10), new(10), new::<29, i32>(20))]
    #[case(new(10), new(10), new::<30, i32>(20))]
    #[case(new(10), new(10), new::<31, i32>(20))]
    #[case(new(10), new(10), new::<32, i32>(20))]
    #[case(new(10), new(10), new::<33, i32>(20))]
    #[case(new(10), new(10), new::<34, i32>(20))]
    #[case(new(10), new(10), new::<35, i32>(20))]
    #[case(new(10), new(10), new::<36, i32>(20))]
    #[case(new(10), new(10), new::<37, i32>(20))]
    #[case(new(10), new(10), new::<38, i32>(20))]
    #[case(new(10), new(10), new::<1, i64>(20))]
    #[case(new(10), new(10), new::<2, i64>(20))]
    #[case(new(10), new(10), new::<3, i64>(20))]
    #[case(new(10), new(10), new::<4, i64>(20))]
    #[case(new(10), new(10), new::<5, i64>(20))]
    #[case(new(10), new(10), new::<6, i64>(20))]
    #[case(new(10), new(10), new::<7, i64>(20))]
    #[case(new(10), new(10), new::<8, i64>(20))]
    #[case(new(10), new(10), new::<9, i64>(20))]
    #[case(new(10), new(10), new::<10, i64>(20))]
    #[case(new(10), new(10), new::<11, i64>(20))]
    #[case(new(10), new(10), new::<12, i64>(20))]
    #[case(new(10), new(10), new::<13, i64>(20))]
    #[case(new(10), new(10), new::<14, i64>(20))]
    #[case(new(10), new(10), new::<15, i64>(20))]
    #[case(new(10), new(10), new::<16, i64>(20))]
    #[case(new(10), new(10), new::<17, i64>(20))]
    #[case(new(10), new(10), new::<18, i64>(20))]
    #[case(new(10), new(10), new::<19, i64>(20))]
    #[case(new(10), new(10), new::<20, i64>(20))]
    #[case(new(10), new(10), new::<21, i64>(20))]
    #[case(new(10), new(10), new::<22, i64>(20))]
    #[case(new(10), new(10), new::<23, i64>(20))]
    #[case(new(10), new(10), new::<24, i64>(20))]
    #[case(new(10), new(10), new::<25, i64>(20))]
    #[case(new(10), new(10), new::<26, i64>(20))]
    #[case(new(10), new(10), new::<27, i64>(20))]
    #[case(new(10), new(10), new::<28, i64>(20))]
    #[case(new(10), new(10), new::<29, i64>(20))]
    #[case(new(10), new(10), new::<30, i64>(20))]
    #[case(new(10), new(10), new::<31, i64>(20))]
    #[case(new(10), new(10), new::<32, i64>(20))]
    #[case(new(10), new(10), new::<33, i64>(20))]
    #[case(new(10), new(10), new::<34, i64>(20))]
    #[case(new(10), new(10), new::<35, i64>(20))]
    #[case(new(10), new(10), new::<36, i64>(20))]
    #[case(new(10), new(10), new::<37, i64>(20))]
    #[case(new(10), new(10), new::<38, i64>(20))]
    #[case(new(10), new(10), new::<1, i128>(20))]
    #[case(new(10), new(10), new::<2, i128>(20))]
    #[case(new(10), new(10), new::<3, i128>(20))]
    #[case(new(10), new(10), new::<4, i128>(20))]
    #[case(new(10), new(10), new::<5, i128>(20))]
    #[case(new(10), new(10), new::<6, i128>(20))]
    #[case(new(10), new(10), new::<7, i128>(20))]
    #[case(new(10), new(10), new::<8, i128>(20))]
    #[case(new(10), new(10), new::<9, i128>(20))]
    #[case(new(10), new(10), new::<10, i128>(20))]
    #[case(new(10), new(10), new::<11, i128>(20))]
    #[case(new(10), new(10), new::<12, i128>(20))]
    #[case(new(10), new(10), new::<13, i128>(20))]
    #[case(new(10), new(10), new::<14, i128>(20))]
    #[case(new(10), new(10), new::<15, i128>(20))]
    #[case(new(10), new(10), new::<16, i128>(20))]
    #[case(new(10), new(10), new::<17, i128>(20))]
    #[case(new(10), new(10), new::<18, i128>(20))]
    #[case(new(10), new(10), new::<19, i128>(20))]
    #[case(new(10), new(10), new::<20, i128>(20))]
    #[case(new(10), new(10), new::<21, i128>(20))]
    #[case(new(10), new(10), new::<22, i128>(20))]
    #[case(new(10), new(10), new::<23, i128>(20))]
    #[case(new(10), new(10), new::<24, i128>(20))]
    #[case(new(10), new(10), new::<25, i128>(20))]
    #[case(new(10), new(10), new::<26, i128>(20))]
    #[case(new(10), new(10), new::<27, i128>(20))]
    #[case(new(10), new(10), new::<28, i128>(20))]
    #[case(new(10), new(10), new::<29, i128>(20))]
    #[case(new(10), new(10), new::<30, i128>(20))]
    #[case(new(10), new(10), new::<31, i128>(20))]
    #[case(new(10), new(10), new::<32, i128>(20))]
    #[case(new(10), new(10), new::<33, i128>(20))]
    #[case(new(10), new(10), new::<34, i128>(20))]
    #[case(new(10), new(10), new::<35, i128>(20))]
    #[case(new(10), new(10), new::<36, i128>(20))]
    #[case(new(10), new(10), new::<37, i128>(20))]
    #[case(new(10), new(10), new::<38, i128>(20))]
    fn _test_mul<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>,
        #[case] y: Q<A, B, default_engine::DefaultEngine>,
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>)
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x * y).unwrap();
        assert_eq!(result, correct_result);
    }

    #[rstest]
    #[case(new(10), new(10), new::<1, u8>(10))]
    fn _test_div<const A: u8, B>(
        #[case] x: Q<A, B, default_engine::DefaultEngine>,
        #[case] y: Q<A, B, default_engine::DefaultEngine>,
        #[case] correct_result: Q<A, B, default_engine::DefaultEngine>)
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection {
        let result: Q<A, B, default_engine::DefaultEngine> = (x / y).unwrap();
        assert_eq!(result, correct_result);
    }
}