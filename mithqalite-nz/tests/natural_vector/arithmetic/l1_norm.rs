// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use mithqalite_base::num::arithmetic::traits::{Height, L1Norm};
use mithqalite_base::num::logic::traits::SignificantBits;
use mithqalite_base::vector::Vector;
use mithqalite_nz::integer_vector::IntegerVector;
use mithqalite_nz::natural::Natural;
use mithqalite_nz::natural_vector::NaturalVector;
use mithqalite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_l1_norm() {
    let test = |s, out, bits| {
        let v = NaturalVector::from_str(s).unwrap();
        let norm = v.to_l1_norm();
        assert_eq!(norm.to_string(), out);
        assert_eq!(v.l1_norm_significant_bits(), bits);
        assert_eq!(v.into_l1_norm(), norm);
    };
    test("()", "0", 0);
    test("(1, 5, 2)", "8", 4);
}

#[test]
fn l1_norm_properties() {
    natural_vector_gen().test_properties(|v| {
        let norm = v.to_l1_norm();
        assert_eq!(v.clone().into_l1_norm(), norm);
        let bits = v.l1_norm_significant_bits();
        assert_eq!(norm.significant_bits(), bits);
        assert_eq!(v.elements.iter().sum::<Natural>(), norm);
        // The norm is at least the height, and at most the dimension times the height.
        let height = v.to_height();
        assert!(height <= norm);
        assert!(norm <= &height * Natural::from(v.dimension()));
        // The `Natural` and `Integer` vectors have the same norm.
        assert_eq!(IntegerVector::from(v.clone()).to_l1_norm(), norm);
    });
}
