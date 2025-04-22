use crate::core::cs_k::*;
use crate::core::tr_brandable::*;
use crate::core::tr_sign_introspection::*;
use num_traits::int::PrimInt;

/// Creates a new instance of `K<A, B>` from a given integer value `v` of type `B`.
/// This function is intended to initialize a `K<A, B>` object where `A` and `B` are compile-time constants or types.
/// 
/// # Type Parameters
/// 
/// - `A`: A compile-time constant representing the first dimension of the `K` type.
/// - `B`: A type that implements `PrimInt` and `Brandable`. This type will be used for the integer value `v`.
/// - `C`: A compile-time constant representing the second dimension of the `K` type, used in the `K::new_from_int` method.
/// - `D`: A type that implements `PrimInt` and `Brandable`. This type represents the type of the input value `v` passed into the function.
/// 
/// # Arguments
/// 
/// - `v`: The integer value of type `B` to be converted and used in the creation of the `K<A, B>` object. This value will be used to initialize the new `K` object.
/// 
/// # Returns
/// 
/// Returns a `KResult<K<A, B>>`, which is a result type that wraps either a `K<A, B>` object if successful or an `Error` if an issue occurs.
pub fn k_int<const A: u8, B: PrimInt, const C: u8, D: PrimInt + Brandable + SignIntrospection>(v: B) -> KResult<K::<C, D>> {
    K::new_from_int::<A, B>(v)
}

impl<const A: u8, B: PrimInt + Brandable + SignIntrospection> K<A, B> {
    pub fn new_from_int<const C: u8, D: PrimInt>(v: D) -> KResult<Self> {
        let v: B = B::from(v).ok_or(KError::ConversionFailure)?;
        let v: K<C, B> = k(v);
        let v: K<A, B> = v.cast()?;
        Ok(v)
    }
}