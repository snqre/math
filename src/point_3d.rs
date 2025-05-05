use crate::num::int;
use crate::num::int_introspection;
use crate::num::precision;
use crate::num::q;
use crate::num::default_engine;

pub type Point3DQ1U8 = Point3DQ1<u8>;
pub type Point3DQ2U8 = Point3DQ2<u8>;

pub type Point3DQ1<T> = Point3D<1, T, default_engine::DefaultEngine>;
pub type Point3DQ2<T> = Point3D<2, T, default_engine::DefaultEngine>;
pub type Point3DQ3<T> = Point3D<3, T, default_engine::DefaultEngine>;
pub type Point3DQ4<T> = Point3D<4, T, default_engine::DefaultEngine>;
pub type Point3DQ5<T> = Point3D<5, T, default_engine::DefaultEngine>;
pub type Point3DQ6<T> = Point3D<6, T, default_engine::DefaultEngine>;
pub type Point3DQ7<T> = Point3D<7, T, default_engine::DefaultEngine>;
pub type Point3DQ8<T> = Point3D<8, T, default_engine::DefaultEngine>;
pub type Point3DQ9<T> = Point3D<9, T, default_engine::DefaultEngine>;
pub type Point3DQ10<T> = Point3D<10, T, default_engine::DefaultEngine>;
pub type Point3DQ11<T> = Point3D<11, T, default_engine::DefaultEngine>;
pub type Point3DQ12<T> = Point3D<12, T, default_engine::DefaultEngine>;
pub type Point3DQ13<T> = Point3D<13, T, default_engine::DefaultEngine>;
pub type Point3DQ14<T> = Point3D<14, T, default_engine::DefaultEngine>;
pub type Point3DQ15<T> = Point3D<15, T, default_engine::DefaultEngine>;

pub type Result<T> = ::core::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::thiserror::Error)]
pub enum Error {
    #[error("")]
    Q(#[from] q::Error)
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Point3D<const A: u8, B, C> where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    C: q::Engine, 
    precision::Precision<A>: precision::Compatible {
    _x: q::Q<A, B, C>,
    _y: q::Q<A, B, C>,
    _z: q::Q<A, B, C>
}

pub fn new<const A: u8, B, C>(
    x: &q::Q<A, B, C>,
    y: &q::Q<A, B, C>,
    z: &q::Q<A, B, C>
) -> Point3D<A, B, C> 
where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    C: q::Engine, 
    precision::Precision<A>: precision::Compatible {
    Point3D {
        _x: x.clone(),
        _y: y.clone(),
        _z: z.clone()
    }
}

pub fn default<const A: u8, B>() -> Point3D<A, B, default_engine::DefaultEngine> 
where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    precision::Precision<A>: precision::Compatible {
    let x: q::Q<A, B, default_engine::DefaultEngine> = q::new(0 as u8);
    let y: q::Q<A, B, default_engine::DefaultEngine> = q::new(0 as u8);
    let z: q::Q<A, B, default_engine::DefaultEngine> = q::new(0 as u8);
    Point3D {
        _x: x,
        _y: y,
        _z: z
    }
}
 
impl<const A: u8, B, C> Point3D<A, B, C>
where
    B: int::Int,
    B: int_introspection::IntIntrospection,
    C: q::Engine,
    precision::Precision<A>: precision::Compatible {
    
    pub fn distance_between(&self, rhs: &Self) -> Result<q::Q<A, B, C>> {
        let x_0: &q::Q<A, B, C> = &self._x;
        let y_0: &q::Q<A, B, C> = &self._y;
        let z_0: &q::Q<A, B, C> = &self._z;
        let x_1: &q::Q<A, B, C> = &rhs._x;
        let y_1: &q::Q<A, B, C> = &rhs._y;
        let z_1: &q::Q<A, B, C> = &rhs._z;
        let dx: &q::Q<A, B, C> = &(x_0 - x_1)?;
        let dx: &q::Q<A, B, C> = &(dx * dx)?;
        let dy: &q::Q<A, B, C> = &(y_0 - y_1)?;
        let dy: &q::Q<A, B, C> = &(dy * dy)?;
        let dz: &q::Q<A, B, C> = &(z_0 - z_1)?;
        let dz: &q::Q<A, B, C> = &(dz * dz)?;
        let sum = ((dx + dy)? + dz)?.sqrt()?;
        Ok(sum)
    }
}