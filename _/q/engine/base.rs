use super::*;

pub trait Handler 
where
    Self: muldiv::Handler {

    #[inline]
    fn add<T>(
        &self, 
        x: semantic_fixed::Num<T>,
        y: semantic_fixed::Num<T>
    ) -> Result<semantic_fixed::Num<T>>
    where
        T: num::Int {
        x.checked_add(&y).ok_or(Error::Overflow)
    }

    #[inline]
    fn sub<T>(
        &self, 
        x: semantic_fixed::Num<T>,
        y: semantic_fixed::Num<T>
    ) -> Result<semantic_fixed::Num<T>>
    where
        T: num::Int {
        x.checked_sub(&y).ok_or(Error::Underflow)
    }

    #[inline]
    fn mul<const A: u8, B>(
        &self,
        x: semantic_fixed::Num<B>,
        y: semantic_fixed::Num<B>
    ) -> Result<semantic_fixed::Num<B>>
    where
        B: num::Int {
        self.muldiv(x, y, scale::into::<A, _>())
    }




    #[inline]
    fn div<const A: u8, B>(&self, x: int::Fixed<B>, y: int::Fixed<B>) -> Result<int::Fixed<B>> 
    where 
    B: int::Int {
        let scale: u128 = self.scale::<A, u128>();
        if scale.is_power_of_two() {
            let ret: B = x << scale.trailing_zeros().try_into().unwrap();
            return Ok(ret)
        }
        let scale: B = unsafe {
            B::from(scale).unwrap_unchecked()
        };
        self.muldiv(x, scale, y)
    }
}




