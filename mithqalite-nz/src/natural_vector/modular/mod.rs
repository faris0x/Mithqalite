// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`BalancedMod`](mithqalite_base::num::modular::traits::BalancedMod), which
/// reduces every element of a vector to the representative closest to zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](mithqalite_base::num::modular::traits::BalancedModPowerOf2), which reduce
/// every element of a vector to the representative closest to zero modulo a power of 2.
pub mod balanced_mod_power_of_2;
/// Implementations of [`ModAbsSquared`](mithqalite_base::num::modular::traits::ModAbsSquared), the
/// squared norm of a vector modulo a number.
pub mod mod_abs_squared;
/// Implementations of [`ModAdd`](mithqalite_base::num::modular::traits::ModAdd) and
/// [`ModAddAssign`](mithqalite_base::num::modular::traits::ModAddAssign), for adding vectors modulo
/// a number.
pub mod mod_add;
/// Implementations of [`ModAddDot`](mithqalite_base::num::modular::traits::ModAddDot) and
/// [`ModAddDotAssign`](mithqalite_base::num::modular::traits::ModAddDotAssign), for adding the dot
/// product of two vectors to a number modulo a number.
pub mod mod_add_dot;
/// Implementations of
/// [`ModAddDotReversed`](mithqalite_base::num::modular::traits::ModAddDotReversed) and
/// [`ModAddDotReversedAssign`](mithqalite_base::num::modular::traits::ModAddDotReversedAssign), for
/// adding the dot product of a vector and the reverse of another to a number modulo a number.
pub mod mod_add_dot_reversed;
/// Implementations of [`ModAddMul`](mithqalite_base::num::modular::traits::ModAddMul) and
/// [`ModAddMulAssign`](mithqalite_base::num::modular::traits::ModAddMulAssign), for adding a scalar
/// multiple of one vector to another modulo a number.
pub mod mod_add_mul;
/// Implementations of [`ModAddMulShl`](mithqalite_base::num::modular::traits::ModAddMulShl) and
/// [`ModAddMulShlAssign`](mithqalite_base::num::modular::traits::ModAddMulShlAssign), for adding a
/// scalar multiple of one vector, shifted left, to another modulo a number.
pub mod mod_add_mul_shl;
/// Implementations of [`ModCross`](mithqalite_base::num::modular::traits::ModCross) and
/// [`ModCrossAssign`](mithqalite_base::num::modular::traits::ModCrossAssign), for computing the
/// cross product of two 3-dimensional vectors modulo a number.
pub mod mod_cross;
/// Dividing every element of a vector by a scalar modulo $m$, by implementations of
/// [`ModDiv`](mithqalite_base::num::modular::traits::ModDiv).
pub mod mod_div;
/// Implementations of [`ModDot`](mithqalite_base::num::modular::traits::ModDot), the dot product of
/// two vectors modulo a number.
pub mod mod_dot;
/// Implementations of [`ModDotReversed`](mithqalite_base::num::modular::traits::ModDotReversed), the
/// dot product of a vector and the reverse of another modulo a number.
pub mod mod_dot_reversed;
/// Implementations of
/// [`ModElementProduct`](mithqalite_base::num::modular::traits::ModElementProduct), the product of
/// the elements modulo a number.
pub mod mod_element_product;
/// Implementations of [`ModElementSum`](mithqalite_base::num::modular::traits::ModElementSum), the
/// sum of the elements modulo a number.
pub mod mod_element_sum;
/// Inverting every element of a vector modulo $m$, by implementations of
/// [`ModEntrywiseInverse`](mithqalite_base::num::modular::traits::ModEntrywiseInverse).
pub mod mod_entrywise_inverse;
/// Multiplying two vectors element by element modulo $m$, by implementations of
/// [`ModEntrywiseMul`](mithqalite_base::num::modular::traits::ModEntrywiseMul) and
/// [`ModEntrywiseMulAssign`](mithqalite_base::num::modular::traits::ModEntrywiseMulAssign).
pub mod mod_entrywise_mul;
/// Implementations of
/// [`ModGeneralizedCross`](mithqalite_base::num::modular::traits::ModGeneralizedCross), for
/// computing the generalized cross product of $n - 1$ vectors of dimension $n$ modulo a number.
pub mod mod_generalized_cross;
/// An implementation of [`ModIsReduced`](mithqalite_base::num::modular::traits::ModIsReduced), which
/// checks whether every element of a vector is less than a given modulus.
pub mod mod_is_reduced;
/// Implementations of [`ModMul`](mithqalite_base::num::modular::traits::ModMul) and
/// [`ModMulAssign`](mithqalite_base::num::modular::traits::ModMulAssign), for multiplying a vector
/// by a scalar modulo a number.
pub mod mod_mul;
/// Implementations of [`ModNeg`](mithqalite_base::num::modular::traits::ModNeg) and
/// [`ModNegAssign`](mithqalite_base::num::modular::traits::ModNegAssign), for negating a vector
/// modulo a number.
pub mod mod_neg;
/// Implementations of [`Mod`](mithqalite_base::num::modular::traits::Mod),
/// [`ModAssign`](mithqalite_base::num::modular::traits::ModAssign), [`Rem`](core::ops::Rem), and
/// [`RemAssign`](core::ops::RemAssign), traits for reducing every element of a vector modulo a
/// number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](mithqalite_base::num::modular::traits::ModPowerOf2) and
/// [`ModPowerOf2Assign`](mithqalite_base::num::modular::traits::ModPowerOf2Assign), which reduce
/// every element of a vector modulo a power of 2.
pub mod mod_power_of_2;
/// Implementations of
/// [`ModPowerOf2AbsSquared`](mithqalite_base::num::modular::traits::ModPowerOf2AbsSquared), the
/// squared norm of a vector modulo a power of 2.
pub mod mod_power_of_2_abs_squared;
/// Implementations of [`ModPowerOf2Add`](mithqalite_base::num::modular::traits::ModPowerOf2Add) and
/// [`ModPowerOf2AddAssign`](mithqalite_base::num::modular::traits::ModPowerOf2AddAssign), for adding
/// vectors modulo a power of 2.
pub mod mod_power_of_2_add;
/// Implementations of
/// [`ModPowerOf2AddDot`](mithqalite_base::num::modular::traits::ModPowerOf2AddDot) and
/// [`ModPowerOf2AddDotAssign`](mithqalite_base::num::modular::traits::ModPowerOf2AddDotAssign), for
/// adding the dot product of two vectors to a number modulo a power of 2.
pub mod mod_power_of_2_add_dot;
/// Implementations of [`ModPowerOf2AddDotReversed`](
/// mithqalite_base::num::modular::traits::ModPowerOf2AddDotReversed) and
/// [`ModPowerOf2AddDotReversedAssign`](
/// mithqalite_base::num::modular::traits::ModPowerOf2AddDotReversedAssign), for adding the dot
/// product of a vector and the reverse of another to a number modulo a power of 2.
pub mod mod_power_of_2_add_dot_reversed;
/// Implementations of
/// [`ModPowerOf2AddMul`](mithqalite_base::num::modular::traits::ModPowerOf2AddMul) and
/// [`ModPowerOf2AddMulAssign`](mithqalite_base::num::modular::traits::ModPowerOf2AddMulAssign), for
/// adding a scalar multiple of one vector to another modulo a power of 2.
pub mod mod_power_of_2_add_mul;
/// Implementations of
/// [`ModPowerOf2AddMulShl`](mithqalite_base::num::modular::traits::ModPowerOf2AddMulShl) and
/// [`ModPowerOf2AddMulShlAssign`](
/// mithqalite_base::num::modular::traits::ModPowerOf2AddMulShlAssign), for adding a scalar multiple
/// of one vector, shifted left, to another modulo a power of 2.
pub mod mod_power_of_2_add_mul_shl;
/// Implementations of [`ModPowerOf2Cross`](mithqalite_base::num::modular::traits::ModPowerOf2Cross)
/// and [`ModPowerOf2CrossAssign`](mithqalite_base::num::modular::traits::ModPowerOf2CrossAssign),
/// for computing the cross product of two 3-dimensional vectors modulo $2^k$.
pub mod mod_power_of_2_cross;
/// Dividing every element of a vector by a scalar modulo $2^k$, by implementations of
/// [`ModPowerOf2Div`](mithqalite_base::num::modular::traits::ModPowerOf2Div).
pub mod mod_power_of_2_div;
/// Implementations of [`ModPowerOf2Dot`](mithqalite_base::num::modular::traits::ModPowerOf2Dot), the
/// dot product of two vectors modulo a power of 2.
pub mod mod_power_of_2_dot;
/// Implementations of
/// [`ModPowerOf2DotReversed`](mithqalite_base::num::modular::traits::ModPowerOf2DotReversed), the
/// dot product of a vector and the reverse of another modulo a power of 2.
pub mod mod_power_of_2_dot_reversed;
/// An implementation of [`ModPowerOf2ElementProduct`](
/// mithqalite_base::num::modular::traits::ModPowerOf2ElementProduct), the product of the elements
/// modulo a power of 2.
pub mod mod_power_of_2_element_product;
/// An implementation of
/// [`ModPowerOf2ElementSum`](mithqalite_base::num::modular::traits::ModPowerOf2ElementSum), the sum
/// of the elements modulo a power of 2.
pub mod mod_power_of_2_element_sum;
/// Inverting every element of a vector modulo $2^k$, by implementations of
/// [`ModPowerOf2EntrywiseInverse`](
/// mithqalite_base::num::modular::traits::ModPowerOf2EntrywiseInverse).
pub mod mod_power_of_2_entrywise_inverse;
/// Multiplying two vectors element by element modulo $2^k$, by implementations of
/// [`ModPowerOf2EntrywiseMul`](mithqalite_base::num::modular::traits::ModPowerOf2EntrywiseMul) and
/// [`ModPowerOf2EntrywiseMulAssign`](
/// mithqalite_base::num::modular::traits::ModPowerOf2EntrywiseMulAssign).
pub mod mod_power_of_2_entrywise_mul;
/// Implementations of [`ModPowerOf2GeneralizedCross`](
/// mithqalite_base::num::modular::traits::ModPowerOf2GeneralizedCross), for computing the
/// generalized cross product of $n - 1$ vectors of dimension $n$ modulo $2^k$.
pub mod mod_power_of_2_generalized_cross;
/// An implementation of
/// [`ModPowerOf2IsReduced`](mithqalite_base::num::modular::traits::ModPowerOf2IsReduced), which
/// checks whether every element of a vector is less than a given power of 2.
pub mod mod_power_of_2_is_reduced;
/// Implementations of [`ModPowerOf2Mul`](mithqalite_base::num::modular::traits::ModPowerOf2Mul) and
/// [`ModPowerOf2MulAssign`](mithqalite_base::num::modular::traits::ModPowerOf2MulAssign), for
/// multiplying a vector by a scalar modulo a power of 2.
pub mod mod_power_of_2_mul;
/// Implementations of [`ModPowerOf2Neg`](mithqalite_base::num::modular::traits::ModPowerOf2Neg) and
/// [`ModPowerOf2NegAssign`](mithqalite_base::num::modular::traits::ModPowerOf2NegAssign), for
/// negating a vector modulo a power of 2.
pub mod mod_power_of_2_neg;
/// Implementations of [`ModPowerOf2Shl`](mithqalite_base::num::modular::traits::ModPowerOf2Shl) and
/// [`ModPowerOf2ShlAssign`](mithqalite_base::num::modular::traits::ModPowerOf2ShlAssign), for
/// left-shifting a vector (multiplying it by a power of 2) modulo another power of 2.
///
/// # mod_power_of_2_shl
/// ```
/// use core::str::FromStr;
/// use mithqalite_base::num::modular::traits::ModPowerOf2Shl;
/// use mithqalite_nz::natural_vector::NaturalVector;
///
/// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// assert_eq!(
///     v.clone().mod_power_of_2_shl(1u8, 3).to_string(),
///     "(2, 2, 6)"
/// );
/// // Shifting by at least the power zeroes every element, keeping the dimension.
/// assert_eq!(v.mod_power_of_2_shl(3u32, 3).to_string(), "(0, 0, 0)");
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 128, 1)").unwrap())
///         .mod_power_of_2_shl(1u64, 8)
///         .to_string(),
///     "(2, 0, 2)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 1)").unwrap())
///         .mod_power_of_2_shl(100u128, 101)
///         .to_string(),
///     "(1267650600228229401496703205376, 1267650600228229401496703205376)"
/// );
/// ```
///
/// # mod_power_of_2_shl_assign
/// ```
/// use core::str::FromStr;
/// use mithqalite_base::num::modular::traits::ModPowerOf2ShlAssign;
/// use mithqalite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// v.mod_power_of_2_shl_assign(1u8, 3);
/// assert_eq!(v.to_string(), "(2, 2, 6)");
///
/// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// v.mod_power_of_2_shl_assign(3u32, 3);
/// assert_eq!(v.to_string(), "(0, 0, 0)");
/// ```
pub mod mod_power_of_2_shl;
/// Implementations of [`ModPowerOf2Sub`](mithqalite_base::num::modular::traits::ModPowerOf2Sub) and
/// [`ModPowerOf2SubAssign`](mithqalite_base::num::modular::traits::ModPowerOf2SubAssign), for
/// subtracting vectors modulo a power of 2.
pub mod mod_power_of_2_sub;
/// Implementations of
/// [`ModPowerOf2SubDot`](mithqalite_base::num::modular::traits::ModPowerOf2SubDot) and
/// [`ModPowerOf2SubDotAssign`](mithqalite_base::num::modular::traits::ModPowerOf2SubDotAssign), for
/// subtracting the dot product of two vectors from a number modulo a power of 2.
pub mod mod_power_of_2_sub_dot;
/// Implementations of [`ModPowerOf2SubDotReversed`](
/// mithqalite_base::num::modular::traits::ModPowerOf2SubDotReversed) and
/// [`ModPowerOf2SubDotReversedAssign`](
/// mithqalite_base::num::modular::traits::ModPowerOf2SubDotReversedAssign), for subtracting the dot
/// product of a vector and the reverse of another from a number modulo a power of 2.
pub mod mod_power_of_2_sub_dot_reversed;
/// Implementations of
/// [`ModPowerOf2SubMul`](mithqalite_base::num::modular::traits::ModPowerOf2SubMul) and
/// [`ModPowerOf2SubMulAssign`](mithqalite_base::num::modular::traits::ModPowerOf2SubMulAssign), for
/// subtracting a scalar multiple of one vector from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul;
/// Implementations of
/// [`ModPowerOf2SubMulShl`](mithqalite_base::num::modular::traits::ModPowerOf2SubMulShl) and
/// [`ModPowerOf2SubMulShlAssign`](
/// mithqalite_base::num::modular::traits::ModPowerOf2SubMulShlAssign), for subtracting a scalar
/// multiple of one vector, shifted left, from another modulo a power of 2.
pub mod mod_power_of_2_sub_mul_shl;
/// Implementations of [`ModPowerOf2Sum`](mithqalite_base::num::modular::traits::ModPowerOf2Sum), for
/// adding up vectors entry by entry modulo a power of 2.
pub mod mod_power_of_2_sum;
/// Implementations of [`ModShl`](mithqalite_base::num::modular::traits::ModShl) and
/// [`ModShlAssign`](mithqalite_base::num::modular::traits::ModShlAssign), for left-shifting a vector
/// (multiplying it by a power of 2) modulo a number.
///
/// # mod_shl
/// ```
/// use core::str::FromStr;
/// use mithqalite_base::num::modular::traits::ModShl;
/// use mithqalite_nz::natural::Natural;
/// use mithqalite_nz::natural_vector::NaturalVector;
///
/// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// assert_eq!(
///     v.clone().mod_shl(1u8, Natural::from(7u32)).to_string(),
///     "(3, 2, 6)"
/// );
/// // The modulus need not be odd, so elements can become zero, keeping the dimension.
/// assert_eq!(
///     v.mod_shl(3u32, Natural::from(8u32)).to_string(),
///     "(0, 0, 0)"
/// );
/// assert_eq!(
///     (&NaturalVector::from_str("(1, 1)").unwrap())
///         .mod_shl(
///             100u64,
///             Natural::from_str("1000000000000000000000000000000").unwrap()
///         )
///         .to_string(),
///     "(267650600228229401496703205376, 267650600228229401496703205376)"
/// );
/// ```
///
/// # mod_shl_assign
/// ```
/// use core::str::FromStr;
/// use mithqalite_base::num::modular::traits::ModShlAssign;
/// use mithqalite_nz::natural::Natural;
/// use mithqalite_nz::natural_vector::NaturalVector;
///
/// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
/// v.mod_shl_assign(1u8, Natural::from(7u32));
/// assert_eq!(v.to_string(), "(3, 2, 6)");
/// ```
pub mod mod_shl;
/// Implementations of [`ModSub`](mithqalite_base::num::modular::traits::ModSub) and
/// [`ModSubAssign`](mithqalite_base::num::modular::traits::ModSubAssign), for subtracting vectors
/// modulo a number.
pub mod mod_sub;
/// Implementations of [`ModSubDot`](mithqalite_base::num::modular::traits::ModSubDot) and
/// [`ModSubDotAssign`](mithqalite_base::num::modular::traits::ModSubDotAssign), for subtracting the
/// dot product of two vectors from a number modulo a number.
pub mod mod_sub_dot;
/// Implementations of
/// [`ModSubDotReversed`](mithqalite_base::num::modular::traits::ModSubDotReversed) and
/// [`ModSubDotReversedAssign`](mithqalite_base::num::modular::traits::ModSubDotReversedAssign), for
/// subtracting the dot product of a vector and the reverse of another from a number modulo a
/// number.
pub mod mod_sub_dot_reversed;
/// Implementations of [`ModSubMul`](mithqalite_base::num::modular::traits::ModSubMul) and
/// [`ModSubMulAssign`](mithqalite_base::num::modular::traits::ModSubMulAssign), for subtracting a
/// scalar multiple of one vector from another modulo a number.
pub mod mod_sub_mul;
/// Implementations of [`ModSubMulShl`](mithqalite_base::num::modular::traits::ModSubMulShl) and
/// [`ModSubMulShlAssign`](mithqalite_base::num::modular::traits::ModSubMulShlAssign), for
/// subtracting a scalar multiple of one vector, shifted left, from another modulo a number.
pub mod mod_sub_mul_shl;
/// Implementations of [`ModSum`](mithqalite_base::num::modular::traits::ModSum), for adding up
/// vectors entry by entry modulo a number.
pub mod mod_sum;
/// [`NaturalVector::multi_crt`](super::NaturalVector::multi_crt), which combines vectors of
/// residues modulo many word-sized moduli by the Chinese remainder theorem.
pub mod multi_crt;
/// [`NaturalVector::multi_mod`](super::NaturalVector::multi_mod), which reduces a vector modulo
/// many word-sized moduli.
pub mod multi_mod;
