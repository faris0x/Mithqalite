// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// In FLINT's fmpzi multiplication and squaring, the paths that trade multiplications for additions
// are only worthwhile when the real and imaginary parts have similar sizes: within 2 limbs (with
// 64-bit limbs) in fmpzi_mul and fmpzi_sqr alike. The tolerance is expressed here in bits so that
// it does not shift when Malachite is built with 32-bit limbs.
pub(crate) const SIZE_BALANCE_BITS: u64 = 2 * 64;

/// Implementations of [`AbsSquared`](mithqalite_base::num::arithmetic::traits::AbsSquared) and
/// [`AbsSquaredAssign`](mithqalite_base::num::arithmetic::traits::AbsSquaredAssign), traits for
/// computing the squared absolute value (norm) of a number.
pub mod abs_squared;
/// Addition of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod add;
/// Implementations of [`CanonicalPrimitivePart`](
/// mithqalite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// mithqalite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// mithqalite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which compute the
/// primitive part of a Gaussian integer in canonical unit form.
pub mod canonical_primitive_part;
/// An implementation of
/// [`CanonicalUnitIPow`](mithqalite_base::num::arithmetic::traits::CanonicalUnitIPow), a trait for
/// finding the power of $i$ that brings a number into canonical unit form.
pub mod canonical_unit_i_pow;
/// Implementations of
/// [`CanonicalizeGaussianUnit`](mithqalite_base::num::arithmetic::traits::CanonicalizeGaussianUnit)
/// and [`CanonicalizeGaussianUnitAssign`](
/// mithqalite_base::num::arithmetic::traits::CanonicalizeGaussianUnitAssign), traits for bringing a
/// number into canonical Gaussian-unit form.
pub mod canonicalize_gaussian_unit;
/// Implementations of
/// [`CanonicalizeUnit`](mithqalite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](mithqalite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// traits for bringing a number into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`Conjugate`](mithqalite_base::num::arithmetic::traits::Conjugate) and
/// [`ConjugateAssign`](mithqalite_base::num::arithmetic::traits::ConjugateAssign), traits for
/// computing the complex conjugate of a number: the sign of the imaginary part is flipped.
pub mod conjugate;
/// Implementations of
/// [`ContentAndPrimitivePart`](mithqalite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// [`Content`](mithqalite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](mithqalite_base::num::arithmetic::traits::PrimitivePart), and
/// [`PrimitivePartAssign`](mithqalite_base::num::arithmetic::traits::PrimitivePartAssign), traits
/// for splitting a number into a scalar and an element with coprime parts.
pub mod content_and_primitive_part;
/// Implementations of [`DivExact`](mithqalite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](mithqalite_base::num::arithmetic::traits::DivExactAssign), traits for dividing
/// two numbers when it's known that the division is exact. Implementations of
/// [`Div`](core::ops::Div), [`DivAssign`](core::ops::DivAssign), and
/// [`CheckedDiv`](mithqalite_base::num::arithmetic::traits::CheckedDiv), traits for dividing two
/// numbers.
pub mod div;
pub mod div_exact;
/// Implementations of [`DivI`](mithqalite_base::num::arithmetic::traits::DivI) and
/// [`DivIAssign`](mithqalite_base::num::arithmetic::traits::DivIAssign), traits for dividing a
/// number by $i$: a clockwise quarter turn.
pub mod div_i;
/// Implementations of [`DivRem`](mithqalite_base::num::arithmetic::traits::DivRem) and
/// [`DivAssignRem`](mithqalite_base::num::arithmetic::traits::DivAssignRem), traits for dividing two
/// numbers and returning the quotient and remainder.
pub mod div_rem;
/// Implementations of [`Gcd`](mithqalite_base::num::number_theory::traits::Gcd) and
/// [`GcdAssign`](mithqalite_base::num::number_theory::traits::GcdAssign), traits for computing the
/// GCD (greatest common divisor) of two numbers.
pub mod gcd;
/// An implementation of [`IsUnit`](mithqalite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring. Implementations of
/// [`Height`](mithqalite_base::num::arithmetic::traits::Height) and
/// [`HeightRef`](mithqalite_base::num::arithmetic::traits::HeightRef), the larger of the magnitudes
/// of a [`GaussianInteger`](super::GaussianInteger)'s parts.
pub mod height;
/// An implementation of [`IsPowerOf2`](mithqalite_base::num::arithmetic::traits::IsPowerOf2), a
/// trait for determining whether a number is an integer power of 2.
pub mod is_power_of_2;
pub mod is_unit;
/// Multiplication of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod mul;
/// Implementations of [`MulI`](mithqalite_base::num::arithmetic::traits::MulI) and
/// [`MulIAssign`](mithqalite_base::num::arithmetic::traits::MulIAssign), traits for multiplying a
/// number by $i$: a counterclockwise quarter turn.
pub mod mul_i;
/// Implementations of [`MulIPow`](mithqalite_base::num::arithmetic::traits::MulIPow) and
/// [`MulIPowAssign`](mithqalite_base::num::arithmetic::traits::MulIPowAssign), traits for
/// multiplying a number by a power of $i$: a multiple of a quarter turn.
pub mod mul_i_pow;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](mithqalite_base::num::arithmetic::traits::NegAssign) for
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger), negating both the real and
/// imaginary parts.
pub mod neg;
/// Implementations of [`Pow`](mithqalite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](mithqalite_base::num::arithmetic::traits::PowAssign), traits for raising a number
/// to a power.
pub mod pow;
/// An implementation of [`PowerOf2`](mithqalite_base::num::arithmetic::traits::PowerOf2), a trait
/// for computing a power of 2.
pub mod power_of_2;
/// Implementations of [`Rem`](core::ops::Rem) and [`RemAssign`](core::ops::RemAssign), traits for
/// finding the remainder of two numbers, subject to the nearest-quotient rounding used by Gaussian
/// integer division.
pub mod rem;
/// Implementations of [`CheckedRoot`](mithqalite_base::num::arithmetic::traits::CheckedRoot), a
/// trait for taking the $n$th root of a number when it is a perfect $n$th power.
pub mod root;
/// Left-shifting a [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) (multiplying it by
/// a power of 2).
///
/// # shl
/// ```
/// use mithqalite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// let x = GaussianInteger::from_str("3-2i").unwrap();
/// assert_eq!((x.clone() << 3u8).to_string(), "24-16i");
/// assert_eq!((&x << 3u64).to_string(), "24-16i");
/// assert_eq!(
///     (x << 100u32).to_string(),
///     "3802951800684688204490109616128-2535301200456458802993406410752i"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use mithqalite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// let mut x = GaussianInteger::from_str("1+i").unwrap();
/// x <<= 1u8;
/// x <<= 2u16;
/// x <<= 3u32;
/// x <<= 4u64;
/// assert_eq!(x.to_string(), "1024+1024i");
/// ```
pub mod shl;
/// Implementations of [`CheckedSqrt`](mithqalite_base::num::arithmetic::traits::CheckedSqrt), a
/// trait for taking the square root of a number when it is a perfect square.
pub mod sqrt;
/// Implementations of [`Square`](mithqalite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](mithqalite_base::num::arithmetic::traits::SquareAssign), traits for squaring a
/// number.
pub mod square;
/// Subtraction of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod sub;
