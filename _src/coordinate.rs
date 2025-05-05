use crate::common::util;
use crate::common::int;
use crate::math::q;
use crate::math::engine::default_engine;
use ::core::ops;
use ::core::fmt;
use ::core::cmp;
use ::thiserror as error;

use util::Util as _;


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
    Self: ops::Add<Output = Result<Self>>,
    Self: ops::Sub<Output = Result<Self>>,
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {

    fn x(&self) -> q::Q<A, B, C>;
    
    fn y(&self) -> q::Q<A, B, C>;

    fn direction_to(&self, rhs: &Self) -> Point2DIDirection {
        let dx = match self.x().partial_cmp(&rhs.x()) {
            Some(cmp::Ordering::Less) => 1,
            Some(cmp::Ordering::Greater) => -1,
            _ => 0
        };
        let dy = match self.y().partial_cmp(&rhs.y()) {
            Some(cmp::Ordering::Less) => 1,
            Some(cmp::Ordering::Greater) => -1,
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

    fn distance_between(&self, rhs: &Self) -> Result<q::Q<A, B, C>> {
        let coordinate_0: &Self = self;
        let coordinate_1: &Self = rhs;
        let x_0: q::Q<A, B, C> = coordinate_0.x();
        let y_0: q::Q<A, B, C> = coordinate_0.y();
        let x_1: q::Q<A, B, C> = coordinate_1.x();
        let y_1: q::Q<A, B, C> = coordinate_1.y();
        let dx: q::Q<A, B, C> = (x_0 - x_1)?;
        let dy: q::Q<A, B, C> = (y_0 - y_1)?;
        let dx_sq: q::Q<A, B, C> = (dx.clone() * dx.clone())?;
        let dy_sq: q::Q<A, B, C> = (dy.clone() * dy.clone())?;
        let sum: q::Q<A, B, C> = (dx_sq + dy_sq)?;
        let distance: q::Q<A, B, C> = sum.sqrt()?;
        Ok(distance)
    }
}




pub type Result<T> = ::core::result::Result<T, Error>;
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(error::Error)]
pub enum Error {
    #[error("")]
    Q(#[from] q::QE)
}
#[derive(Clone)]
pub struct Coordinates<const A: u8, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {
    _x: q::Q<A, B, C>,
    _y: q::Q<A, B, C>
}

pub fn new<const A: u8, B, C>(
    x: q::Q<A, B, C>, 
    y: q::Q<A, B, C>) -> Point2DI<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {
    Point2DI {
        _x: x,
        _y: y
    }
}

impl<const A: u8, B, C> Point2DI<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {
    
    pub fn distance_between(&self, rhs: &Self) -> Result<q::Q<A, B, C>> {
        let coordinate_0: &Self = self;
        let coordinate_1: &Self = rhs;
        let x_0: q::Q<A, B, C> = coordinate_0._x.clone();
        let y_0: q::Q<A, B, C> = coordinate_0._y.clone();
        let x_1: q::Q<A, B, C> = coordinate_1._x.clone();
        let y_1: q::Q<A, B, C> = coordinate_1._y.clone();
        let dx: q::Q<A, B, C> = (x_0 - x_1)?;
        let dy: q::Q<A, B, C> = (y_0 - y_1)?;
        let dx_sq: q::Q<A, B, C> = (dx.clone() * dx.clone())?;
        let dy_sq: q::Q<A, B, C> = (dy.clone() * dy.clone())?;
        let sum: q::Q<A, B, C> = (dx_sq + dy_sq)?;
        let distance: q::Q<A, B, C> = sum.sqrt()?;
        Ok(distance)
    }
}

impl<const A: u8, B, C> fmt::Debug for Point2DI<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl<const A: u8, B, C> fmt::Display for Point2DI<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

//
//
// add 2 coordinates together to get a new coordinate which is
// moved more towards the added direction
impl<const A: u8, B, C> ops::Add for Point2DI<A, B, C>
where
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let coordinate_0: Self = self;
        let coordinate_1: Self = rhs;
        let x_0: q::Q<A, B, C> = coordinate_0._x;
        let y_0: q::Q<A, B, C> = coordinate_0._y;
        let x_1: q::Q<A, B, C> = coordinate_1._x;
        let y_1: q::Q<A, B, C> = coordinate_1._y;
        let x_2 = (x_0 + x_1)?;
        let y_2 = (y_0 + y_1)?;
        let result: Point2DI<A, B, C> = new(x_2, y_2);
        Ok(result)
    }
}

impl<const A: u8, B, C> ops::Sub for Point2DI<A, B, C> 
where 
    B: int::Int,
    B: int::Introspection,
    C: q::QEngine {
    type Output = Result<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let coordinate_0: Self = self;
        let coordinate_1: Self = rhs;
        let x_0: q::Q<A, B, C> = coordinate_0._x;
        let y_0: q::Q<A, B, C> = coordinate_0._y;
        let x_1: q::Q<A, B, C> = coordinate_1._x;
        let y_1: q::Q<A, B, C> = coordinate_1._y;
        let x_2 = (x_0 - x_1)?;
        let y_2 = (y_0 - y_1)?;
        let result: Point2DI<A, B, C> = new(x_2, y_2);
        Ok(result)
    }
}