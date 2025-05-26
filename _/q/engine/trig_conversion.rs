use super::*;

pub trait Handler
where
    Self: muldiv::Handler {
    #[inline]
    fn to_radian<const A: u8, B>(&self, angle: semantic::Degree<semantic::Fixed<B>>) -> Result<semantic::Radian<semantic::Fixed<B>>> 
    where 
        B: num::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        let scale: B = self.scale::<A, _>();
        let pi: B = self.pi::<A, _>();
        self.muldiv(angle, pi, n * scale)
    }
    
    #[inline]
    fn to_degree<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::Degree<semantic::Fixed<B>>> 
    where 
        B: num::Int {
        let n: u8 = 180;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        let scale: B = self.scale::<A, _>();
        let pi: B = self.pi::<A, _>();
        self.muldiv(angle, n * scale, pi)
    }
}

impl<T> Handler for T
where
    T: super::Handler
{}