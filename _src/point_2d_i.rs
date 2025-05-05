use crate::common::int::Int;
use crate::common::int::Introspection;
use crate::math::q::*;
use ::core::ops::Add;
use ::core::ops::Sub;
use ::core::cmp::Ordering;

pub type Point2DIR<T> = Result<T, Point2DIE>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::thiserror::Error)]
pub enum Point2DIE {
    #[error("")]
    Q(#[from] QE)
}

pub enum Point2DIDirection {
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
    Right,
    TopRight
}

pub trait Point2DI<const A: u8, B, C> 
where
    Self: Sized,
    Self: Add<Output = Point2DIR<Self>>,
    Self: Sub<Output = Point2DIR<Self>>,
    B: Int,
    B: Introspection,
    C: QEngine {

    fn x(&self) -> Q<A, B, C>;
    
    fn y(&self) -> Q<A, B, C>;

    fn direction_to(&self, rhs: &Self) -> Point2DIDirection {
        let dx = match self.x().partial_cmp(&rhs.x()) {
            Some(Ordering::Less) => 1,
            Some(Ordering::Greater) => -1,
            _ => 0
        };
        let dy = match self.y().partial_cmp(&rhs.y()) {
            Some(Ordering::Less) => 1,
            Some(Ordering::Greater) => -1,
            _ => 0
        };
        match (dx, dy) {
            (0, 1) => Point2DIDirection::Top,
            (-1, 1) => Point2DIDirection::TopLeft,
            (-1, 0) => Point2DIDirection::Left,
            (-1, -1) => Point2DIDirection::BottomLeft,
            (0, -1) => Point2DIDirection::Bottom,
            (1, -1) => Point2DIDirection::BottomRight,
            (1, 0) => Point2DIDirection::Right,
            (1, 1) => Point2DIDirection::TopRight,
            _ => panic!()
        }
    }

    fn distance_between(&self, rhs: &Self) -> Point2DIR<Q<A, B, C>> {
        let coordinate_0: &Self = self;
        let coordinate_1: &Self = rhs;
        let x_0: Q<A, B, C> = coordinate_0.x();
        let y_0: Q<A, B, C> = coordinate_0.y();
        let x_1: Q<A, B, C> = coordinate_1.x();
        let y_1: Q<A, B, C> = coordinate_1.y();
        let dx: Q<A, B, C> = (x_0 - x_1)?;
        let dy: Q<A, B, C> = (y_0 - y_1)?;
        let dx_sq: Q<A, B, C> = (dx.clone() * dx.clone())?;
        let dy_sq: Q<A, B, C> = (dy.clone() * dy.clone())?;
        let sum: Q<A, B, C> = (dx_sq + dy_sq)?;
        let distance: Q<A, B, C> = sum.sqrt()?;
        Ok(distance)
    }

    fn angle_between(&self, rhs: &Self) -> Point2DIR<Q<A, B, C>> {
        let x_0: Q<A, B, C> = self.x();
        let y_0: Q<A, B, C> = self.y();
        let x_1: Q<A, B, C> = rhs.x();
        let y_1: Q<A, B, C> = rhs.y();
        let dot: Q<A, B, C> = ((x_0 * x_1)? + (y_0 * y_1)?)?;
        let mag_0: Q<A, B, C> = ((x_0 * x_0)? + (y_0 * y_0)?)?.sqrt()?;
        let mag_1: Q<A, B, C> = ((x_1 * x_1)? + (y_1 * y_1)?)?.sqrt()?;
        let cos_theta = (dot / (mag_0 * mag_1)?)?;
        let angle_radian = cos_theta.acos();
    
    }
}