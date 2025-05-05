use crate::common::int;
use crate::coordinate;
use crate::math::q;
use crate::direction_label;
use ::core::fmt;

pub enum FourSidedDirectionLabel {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft
}

impl FourSidedDirectionLabel {
    
}

// measures the distance from on point to another as the
// magniture or trength to that direction, and obviously the
// direction from the start
pub struct Direction<const A: u8, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {
    _start: coordinate::Point2DI<A, B, C>,
    _end: coordinate::Point2DI<A, B, C>,
}

impl Direction<> {

    pub fn to_4_side_direction_label(&self) -> FourSidedDirectionLabel {
        let dx = self._end
    }

    pub fn to_8_direction_label(&self) -> DirectionX8 {

    }
}

impl<const A: u8, B, C> fmt::Debug for Direction<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}