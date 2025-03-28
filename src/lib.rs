#![deny(warnings)]

pub mod point {
    use crate::f_128::*;
    use std::fmt::Debug;
    use std::fmt::Display;

    pub trait PointI<T1: F128I>:
          Sized
        + PointCoreI<T1>
        + Debug
        + Display 
        + Clone
        + Copy 
        {}

    pub trait PointCoreI<T1: F128I>: Sized {
        fn x(&self) -> T1;
        fn y(&self) -> T1;
        fn distance(&self, rhs: Self) -> Option<T1>;
    }

    #[derive(Clone)]
    #[derive(Copy)]
    pub struct Point<T1: F128I> {
        _x: T1,
        _y: T1,
    }

    impl<T1: F128I> PointI<T1> for Point<T1>
        {}

    impl<T1: F128I> Point<T1> {
        pub fn new(x: T1, y: T1) -> Self {
            Self {
                _x: x,
                _y: y
            }
        }
    }

    impl<T1: F128I> PointCoreI<T1> for Point<T1> {
        fn x(&self) -> T1 {
            self._x
        }

        fn y(&self) -> T1 {
            self._y
        }

        fn distance(&self, rhs: Self) -> Option<T1> {
            let dfx: T1 = if self._x > rhs._x {
                (self._x - rhs._x)?
            } else {
                (rhs._x - self._x)?
            };
            let dfy: T1 = if self._y > rhs._y {
                (self._y - rhs._y)?
            } else {
                (rhs._y - self._y)?
            };
            let result: T1 = ((dfx * (dfy * dfy)?)?).sqrt()?;
            Some(result)
        }
    }

    impl<T1: F128I> Debug for Point<T1> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Point({},{})", self._x, self._y)
        }
    }

    impl<T1: F128I> Display for Point<T1> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Point({},{})", self._x, self._y)
        }
    }
}

pub mod f_128 {
    use std::ops::Add;
    use std::ops::Sub;
    use std::ops::Mul;
    use std::ops::Div;
    use std::ops::Shl;
    use std::ops::Shr;
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::cmp::Ordering;

    pub trait F128I: 
          Sized
        + F128CoreI
        + Debug
        + Display
        + Clone
        + Copy
        + Add<Output=Option<Self>>
        + Sub<Output=Option<Self>>
        + Mul<Output=Option<Self>>
        + Div<Output=Option<Self>>
        + Shl<Output=Option<Self>>
        + Shr<Output=Option<Self>>
        + PartialEq
        + Eq
        + PartialOrd
        + Ord 
        {}

    pub trait F128CoreI: Sized {
        fn sqrt(self) -> Option<Self>;
        fn to_new_decimals(self, new_decimals: u32) -> Option<Self>;
    }

    #[derive(Clone)]
    #[derive(Copy)]
    pub struct F128(u128, u32);

    impl F128 {
        pub fn new(value: u128, decimals: u32) -> Self {
            Self(value, decimals)
        }

        pub fn new_zero() -> Self {
            Self(0u128, 0u32)
        }

        fn _normalize(x: Self, y: Self) -> Option<(Self, Self)> {
            let largest_decimals: u32 = Self::_largest_decimals(x, y);
            let result_0: Self = x.to_new_decimals(largest_decimals)?;
            let result_1: Self = y.to_new_decimals(largest_decimals)?;
            Some((result_0, result_1))
        }

        fn _largest_decimals(x: Self, y: Self) -> u32 {
            if x.1 > y.1 {
                return x.1
            }
            if x.1 < y.1 {
                return y.1
            }
            x.1
        }

        pub fn to_new_decimals(value: Self, old_decimals: u32, new_decimals: u32) -> Option<Self> {
            let old_scale: u128 = 10u128.checked_pow(old_decimals)?;
            let new_scale: u128 = 10u128.checked_pow(new_decimals)?;
            let result: u128 = value.0.checked_mul(new_scale)?;
            let result: u128 = result.checked_div(old_scale)?;
            let result: Self = Self(result, new_decimals);
            Some(result)
        }
    }

    impl F128I for F128 
        {}

    impl F128CoreI for F128 {
        fn sqrt(self) -> Option<Self> {
            if self == F128::new_zero() {
                let result: F128 = F128::new_zero();
                return Some(result)
            }
            let mut x: F128 = self;
            let mut y: F128 = (((x + F128::new(100u128, 2u32))?) / F128::new(200u128, 2u32))?;
            while y < x {
                x = y;
                y = (((x + self)? / x)? / F128::new(200u128, 2u32))?;
            }
            Some(x)
        }

        fn to_new_decimals(self, new_decimals: u32) -> Option<Self> {
            Self::to_new_decimals(self, self.1, new_decimals)
            
        }
    }

    impl Debug for F128 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "F128({}, {})", self.0, self.1)
        }
    }

    impl Display for F128 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "F128({}, {})", self.0, self.1)
        }
    }

    impl Add for F128 {
        type Output = Option<Self>;

        fn add(self, rhs: Self) -> Self::Output {
            let result: u128 = self.0.checked_add(rhs.0)?;
            let result: Self = F128::new(result, self.1);
            Some(result)
        }
    }

    impl Sub for F128 {
        type Output = Option<Self>;

        fn sub(self, rhs: Self) -> Self::Output {
            let result: u128 = self.0.checked_sub(rhs.0)?;
            let result: Self = F128(result, self.1);
            Some(result)
        }
    }

    impl Mul for F128 {
        type Output = Option<Self>;

        fn mul(self, rhs: Self) -> Self::Output {
            if self.1 == 0u32 {
                let result: u128 = self.0.checked_mul(rhs.0)?;
                let result: Self = Self::new(result, self.1);
                return Some(result)
            }
            let scale: u128 = 10u128.checked_pow(self.1)?;
            let scale_inv: u128 = 1u128.checked_shl(64u32)?;
            let scale_inv: u128 = scale_inv + scale.checked_div(2u128)?;
            let scale_inv: u128 = scale_inv.checked_div(scale)?;
            let result: u128 = self.0.checked_mul(rhs.0)?;
            let result: u128 = result.checked_mul(scale_inv)?;
            let result: u128 = result.checked_add(1u128.checked_shl(64u32)?)?;
            let result: u128 = result.checked_shr(64u32)?;
            let result: Self = Self(result, self.1);
            Some(result)
        }
    }

    impl Div for F128 {
        type Output = Option<Self>;

        fn div(self, rhs: Self) -> Self::Output {
            if self.1 == 0u32 {
                let result: u128 = self.0.checked_div(rhs.0)?;
                let result: Self = Self(result, self.1);
                return Some(result)
            }
            let scale: u128 = 10u128.checked_pow(self.1)?;
            if !scale.is_power_of_two() {
                let result: u128 = self.0.checked_mul(scale)?;
                let result: u128 = result.checked_div(rhs.0)?;
                let result: Self = Self(result, self.1);
                return Some(result)
            }
            let scale_shift: u32 = scale.trailing_zeros();
            let result: u128 = self.0.checked_shl(scale_shift)?;
            let result: u128 = result.checked_div(rhs.0)?;
            let result: Self = Self(result, self.1);
            Some(result)
        }
    }

    // TODO Double check this implementation.
    impl Shl for F128 {
        type Output = Option<Self>;

        fn shl(self, rhs: Self) -> Self::Output {
            let result: u128 = self.0.checked_shl(rhs.0 as u32)?;
            let result: Self = Self(result, self.1);
            Some(result)
        }
    }

    // TODO Double check this implementation.
    impl Shr for F128 {
        type Output = Option<Self>;

        fn shr(self, rhs: Self) -> Self::Output {
            let result: u128 = self.0.checked_shr(rhs.0 as u32)?;
            let result: Self = Self(result, self.1);
            Some(result)
        }
    }

    impl PartialEq for F128 {
        fn eq(&self, other: &Self) -> bool {
            Self::_normalize(*self, *other)
                .map(|x| {
                    x.0.0 == x.1.0
                })
                .unwrap_or_default()
        }
    }

    impl Eq for F128 
        {}

    impl PartialOrd for F128 {
        fn ge(&self, other: &Self) -> bool {
            self.0 >= other.0
        }

        fn le(&self, other: &Self) -> bool {
            self.0 <= other.0
        }

        fn gt(&self, other: &Self) -> bool {
            self.0 > other.0
        }

        fn lt(&self, other: &Self) -> bool {
            self.0 < other.0
        }

        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for F128 {
        fn clamp(self, min: Self, max: Self) -> Self
            where
                Self: Sized, {
            if self < min {
                return min
            }
            if self > max {
                return max
            }
            self
        }   

        fn max(self, other: Self) -> Self
            where
                Self: Sized, {
            if self > other {
                return self
            }
            if self < other {
                return other
            }
            self
        }

        fn min(self, other: Self) -> Self
            where
                Self: Sized, {
            if self < other {
                return self
            }
            if self > other {
                return other
            }
            self
        }

        fn cmp(&self, other: &Self) -> Ordering {
            use Ordering::*;

            if self > other {
                return Greater
            }
            if self < other {
                return Less
            }
            Equal
        }
    }

    #[cfg(test)]
    mod test {
        use crate::f_128::*;

        #[test]
        fn add() {
            let x: F128 = F128::new(100u128, 2u32);
            let y: F128 = F128::new(200u128, 2u32);
            let result: F128 = (x + y).unwrap();
            let result_ok: F128 = F128::new(300u128, 2u32);
            assert_eq!(result, result_ok);
        }

        #[test]
        fn sub() {
            let x: F128 = F128::new(200u128, 2u32);
            let y: F128 = F128::new(100u128, 2u32);
            let result: F128 = (x - y).unwrap();
            let result_ok: F128 = F128::new(100u128, 2u32);
            assert_eq!(result, result_ok);
        }

        #[test]
        fn mul() {
            let x: F128 = F128::new(200u128, 2u32);
            let y: F128 = F128::new(50u128, 2u32);
            let result: F128 = (x * y).unwrap();
            let result_ok: F128 = F128::new(100u128, 2u32);
            assert_eq!(result, result_ok);
        }

        #[test]
        fn div() {
            let x: F128 = F128::new(200u128, 2u32);
            let y: F128 = F128::new(50u128, 2u32);
            let result: F128 = (x / y).unwrap();
            let result_ok: F128 = F128::new(400u128, 2u32);
            assert_eq!(result, result_ok);
        }
    }
}