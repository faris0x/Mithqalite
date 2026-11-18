// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use mithqalite_base::num::linear_algebra::traits::AreLinearlyIndependent;
use mithqalite_base::test_util::bench::{BenchmarkType, run_benchmark};
use mithqalite_base::test_util::generators::common::{GenConfig, GenMode};
use mithqalite_base::test_util::runner::Runner;
use mithqalite_base::vector::Vector;
use mithqalite_nz::integer_matrix::IntegerMatrix;
use mithqalite_nz::integer_vector::IntegerVector;
use mithqalite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use mithqalite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_are_linearly_independent);
    register_bench!(runner, benchmark_integer_vector_are_linearly_independent);
}

fn rows(matrix: &IntegerMatrix) -> Vec<IntegerVector> {
    matrix
        .rows()
        .map(|row| IntegerVector::from_elements(row.as_slice()))
        .collect()
}

fn demo_integer_vector_are_linearly_independent(gm: GenMode, config: &GenConfig, limit: usize) {
    for matrix in integer_matrix_gen().get(gm, config).take(limit) {
        println!(
            "are_linearly_independent(rows of {}) = {}",
            matrix,
            IntegerVector::are_linearly_independent(&rows(&matrix))
        );
    }
}

fn benchmark_integer_vector_are_linearly_independent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector::are_linearly_independent",
        BenchmarkType::Single,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [("Malachite", &mut |matrix| {
            no_out!(IntegerVector::are_linearly_independent(&rows(&matrix)));
        })],
    );
}
