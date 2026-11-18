// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Bitwise and of [`Integer`](super::Integer)s.
pub mod and;
/// An implementation of [`BitAccess`](mithqalite_base::num::logic::traits::BitAccess), a trait for
/// getting and setting individual bits of a number.
pub mod bit_access;
/// An implementation of [`BitBlockAccess`](mithqalite_base::num::logic::traits::BitBlockAccess), a
/// trait for getting and setting adjacent blocks of bits in a number.
pub mod bit_block_access;
/// An implementation of [`BitConvertible`](mithqalite_base::num::logic::traits::BitConvertible), a
/// trait for extracting all bits from a number or constructing a number from bits.
pub mod bit_convertible;
/// An implementation of [`BitIterable`](mithqalite_base::num::logic::traits::BitIterable), a trait
/// for producing a double-ended iterator over a number's bits.
pub mod bit_iterable;
/// An implementation of [`BitScan`](mithqalite_base::num::logic::traits::BitScan), a trait for
/// finding the next `true` or `false` bit in a number after a provided index.
pub mod bit_scan;
/// A function counting the number of ones in the binary representation of a number.
pub mod checked_count_ones;
/// A function counting the number of zeros in the binary representation of a number.
pub mod checked_count_zeros;
/// An implementation of
/// [`CheckedHammingDistance`](mithqalite_base::num::logic::traits::CheckedHammingDistance), a trait
/// for computing the Hamming distance between two numbers.
pub mod checked_hamming_distance;
/// An implementation of [`LowMask`](mithqalite_base::num::logic::traits::LowMask), a trait for
/// generating a low bit mask (a number in which only the $k$ least-significant bits are 1).
pub mod low_mask;
/// Bitwise negation of [`Integer`](super::Integer)s.
pub mod not;
/// Bitwise or of [`Integer`](super::Integer)s.
pub mod or;
/// An implementation of [`SignificantBits`](mithqalite_base::num::logic::traits::SignificantBits), a
/// trait for determining how many significant bits a number has.
pub mod significant_bits;
/// An implementation of [`TrailingZeros`](mithqalite_base::num::logic::traits::TrailingZeros), a
/// trait for determining the number of zeros that a number ends with when written in binary.
pub mod trailing_zeros;
/// Bitwise xor of [`Integer`](super::Integer)s.
pub mod xor;
