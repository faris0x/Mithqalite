// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Acos`](mithqalite_base::num::trigonometric::traits::Acos) and
/// [`AcosAssign`](mithqalite_base::num::trigonometric::traits::AcosAssign), traits for computing the
/// arccosine of [`Float`](super::Float)s.
pub mod acos;
pub mod acot;
pub mod acsc;
/// Implementations of [`Asec`](mithqalite_base::num::trigonometric::traits::Asec) and
/// [`AsecAssign`](mithqalite_base::num::trigonometric::traits::AsecAssign), traits for computing the
/// arcsecant of [`Float`](super::Float)s.
pub mod asec;
/// Implementations of [`Asin`](mithqalite_base::num::trigonometric::traits::Asin) and
/// [`AsinAssign`](mithqalite_base::num::trigonometric::traits::AsinAssign), traits for computing the
/// arcsine of [`Float`](super::Float)s.
pub mod asin;
/// Implementations of [`Atan`](mithqalite_base::num::trigonometric::traits::Atan) and
/// [`AtanAssign`](mithqalite_base::num::trigonometric::traits::AtanAssign), traits for computing the
/// arctangent of [`Float`](super::Float)s.
pub mod atan;
/// Implementations of [`Atan2`](mithqalite_base::num::trigonometric::traits::Atan2) and
/// [`Atan2Assign`](mithqalite_base::num::trigonometric::traits::Atan2Assign), traits for computing
/// the angle of a point given by two [`Float`](super::Float) coordinates.
pub mod atan2;
/// Implementations of [`Cos`](mithqalite_base::num::trigonometric::traits::Cos) and
/// [`CosAssign`](mithqalite_base::num::trigonometric::traits::CosAssign), traits for computing the
/// cosine of [`Float`](super::Float)s.
pub mod cos;
/// Implementations of [`Cot`](mithqalite_base::num::trigonometric::traits::Cot) and
/// [`CotAssign`](mithqalite_base::num::trigonometric::traits::CotAssign), traits for computing the
/// cotangent of [`Float`](super::Float)s.
pub mod cot;
/// Implementations of [`Csc`](mithqalite_base::num::trigonometric::traits::Csc) and
/// [`CscAssign`](mithqalite_base::num::trigonometric::traits::CscAssign), traits for computing the
/// cosecant of [`Float`](super::Float)s.
pub mod csc;
/// Implementations of [`Sec`](mithqalite_base::num::trigonometric::traits::Sec) and
/// [`SecAssign`](mithqalite_base::num::trigonometric::traits::SecAssign), traits for computing the
/// secant of [`Float`](super::Float)s.
pub mod sec;
/// Implementations of [`Sin`](mithqalite_base::num::trigonometric::traits::Sin) and
/// [`SinAssign`](mithqalite_base::num::trigonometric::traits::SinAssign), traits for computing the
/// sine of [`Float`](super::Float)s.
pub mod sin;
/// Implementations of [`SinCos`](mithqalite_base::num::trigonometric::traits::SinCos) and
/// [`SinCosAssign`](mithqalite_base::num::trigonometric::traits::SinCosAssign), traits for computing
/// the sine and cosine of [`Float`](super::Float)s together.
pub mod sin_cos;
pub mod tan;
