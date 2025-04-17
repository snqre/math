use super::*;

impl<T: _T> Ord for Q<T> {
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else if self < other {
            other
        } else {
            self
        }
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else if self > other {
            other
        } else {
            self
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self > other {
            Ordering::Greater
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}