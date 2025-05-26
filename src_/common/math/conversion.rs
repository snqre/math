use super::*;

#[inline]
pub fn cast<const A: u8, const B: u8, C>(n: int::F<C>) -> Result<int::F<C>> 
where 
    C: int::Int,
    (): precision::Ok<A, C>,
    (): precision::Ok<B, C> {
    if A == B {
        return Ok(n);
    }
    let old_scale: C = scale::into::<A, _>();
    let new_scale: C = scale::into::<B, _>();
    raw::muldiv(n, new_scale, old_scale)
}

#[allow(clippy::zero_prefixed_literal)]
#[cfg(test)]
mod test {
    use ::core::fmt;

    use super::*;

    #[::rstest::rstest]
    #[case(1_00, 1_000000)]
    #[case(0_75, 0_750000)]
    fn cast<T>(#[case] n: T, #[case] ok: T) 
    where
        T: fmt::Debug,
        T: int::Int,
        (): precision::Ok<2, T>,
        (): precision::Ok<6, T> {
        let result: T = super::cast::<2, 6, _>(n).unwrap();
        assert_eq!(result, ok);
    }
}