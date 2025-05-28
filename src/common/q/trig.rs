use super::*;

#[inline]
pub fn deg90<const A: u8, B>() -> Result<semantic_fixed::Degree<B>> where B: int::Int {
    use Error::*;
    
    let degree: B = if B::IS_SIGNED {
        let n: i128 = 90;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        n
    } else {
        let n: u128 = 90;
        let n: B = unsafe {
            B::from(n).unwrap_unchecked()
        };
        n
    };
    let res: B = degree.checked_mul(&scale::get::<A, _>()).ok_or(Overflow)?;
    Ok(res)
}