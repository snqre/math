use crate::precision;
use crate::int;
use crate::q;
use core::ops;
use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(thiserror::Error)]
pub enum Error {

}

#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Point4D<const A: usize, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine,
    precision::Precision<A>: precision::Compatible {
    _x: q::Q<A, B, C>,
    _y: q::Q<A, B, C>,
    _z: q::Q<A, B, C>,
    _t: B
}

pub fn new<const A: usize, B, C>(
    x: q::Q<A, B, C>,
    y: q::Q<A, B, C>,
    z: q::Q<A, B, C>,
    t: B
) -> Point4D<A, B, C> 
where
    B: int::Int,
    B: int::Introspection,
    C: q::Engine,
    precision::Precision<A>: precision::Compatible {
    Point4D {
        _x: x,
        _y: y,
        _z: z,
        _t: t
    }
}

toga::blockset! {
    impl<const A: usize, B, C> Point4D<A, B, C>
    where
        B: fmt::Debug,
        B: int::Int,
        B: int::Introspection,
        C: q::Engine,
        precision::Precision<A>: precision::Compatible;

    fmt::Debug {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Point4D")
                .field("x", &self._x)
                .field("y", &self._y)
                .field("z", &self._z)
                .field("t", &self._t)
                .finish()
        }
    }
}

toga::blockset! {
    impl<const A: usize, B, C> Point4D<A, B, C> 
    where
        B: int::Int,
        B: int::Introspection,
        C: q::Engine,
        precision::Precision<A>: precision::Compatible;

    // a distnce between the two points in space and time
    pub fn distance_between_in_space_and_time() {

    }
    
    pub fn time_between() {

    }
    
    pub fn distance_between() {

    }

    ops::Add {
        type Output = Result<Self>;

        fn add(self, rhs: Self) -> Self::Output {
            
        }
    }

    ops::Sub {
        type Output = Result<Self>;

        fn sub(self, rhs: Self) -> Self::Output {
            
        }
    }
}