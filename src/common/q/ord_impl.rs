use super::*;

impl<const A: u8, B, C> Ord for Q<A, B, C>
where
    B: int::Int,
    C: Engine {
    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized {
        if self < min {
            return min;
        }
        if self > max {
            return max;
        }
        self
    }

    #[inline]
    fn max(self, other: Self) -> Self
    where
        Self: Sized {
        if self < other {
            return other;
        }
        self
    }

    #[inline]
    fn min(self, other: Self) -> Self
    where
        Self: Sized {
        if self > other {
            return other;
        }
        self
    }

    #[inline]
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        if self < other {
            return ::core::cmp::Ordering::Less;
        }
        if self > other {
            return ::core::cmp::Ordering::Greater;
        }
        ::core::cmp::Ordering::Equal
    }
}