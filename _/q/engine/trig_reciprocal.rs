use super::*;

pub trait Handler {
    #[inline]
    fn csc<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::Ratio<semantic::Fixed<B>>> 
    where 
        B: num::Int {
        let out: B = self.sin::<A, _>(angle)?;
        self.inv::<A, _>(out)
    }
    
    #[inline]
    fn sec<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::Ratio<semantic::Fixed<B>>> 
    where 
        B: num::Int {
        let out: B = self.cos::<A, _>(angle)?;
        self.inv::<A, _>(out)
    }
    
    #[inline]
    fn cot<const A: u8, B>(&self, angle: semantic::Radian<semantic::Fixed<B>>) -> Result<semantic::Ratio<semantic::Fixed<B>>> 
    where 
        B: num::Int {
        let out: B = self.tan::<A, _>(angle)?;
        self.inv::<A, _>(out)
    }

    #[inline]
    fn inv<const A: u8, B>(&self, n: B) -> Result<B> 
    where
        B: num::Int {
        let scale: B = self.scale::<A, _>();
        self.div::<A, _>(scale, n)
    }
}

impl<T> Handler for T
where
    T: super::Handler
{}