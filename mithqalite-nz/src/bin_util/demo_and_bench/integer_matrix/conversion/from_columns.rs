// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use mithqalite_base::matrix::Matrix;
use mithqalite_base::num::conversion::traits::ExactFrom;
use mithqalite_base::num::logic::traits::SignificantBits;
use mithqalite_base::test_util::bench::bucketers::Bucketer;
use mithqalite_base::test_util::bench::{BenchmarkType, run_benchmark};
use mithqalite_base::test_util::generators::common::{GenConfig, GenMode};
use mithqalite_base::test_util::runner::Runner;
use mithqalite_base::vector::Vector;
use mithqalite_nz::integer_matrix::IntegerMatrix;
use mithqalite_nz::integer_vector::IntegerVector;
use mithqalite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_from_columns);
    register_demo!(runner, demo_integer_matrix_from_owned_columns);
    register_bench!(
        runner,
        benchmark_integer_matrix_from_columns_evaluation_strategy
    );
}

fn columns_of(m: &IntegerMatrix) -> Vec<IntegerVector> {
    (0..m.width())
        .map(|j| {
            IntegerVector::from_owned_elements(
                (0..m.height())
                    .map(|i| m.elements_ref()[usize::exact_from(i * m.width() + j)].clone())
                    .collect(),
            )
        })
        .collect()
}

fn demo_integer_matrix_from_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        let vs = columns_of(&m);
        println!(
            "IntegerMatrix::from_columns({:?}) = {}",
            vs,
            IntegerMatrix::from_columns(&vs)
        );
    }
}

fn demo_integer_matrix_from_owned_columns(gm: GenMode, config: &GenConfig, limit: usize) {
    for m in integer_matrix_gen().get(gm, config).take(limit) {
        let vs = columns_of(&m);
        println!(
            "IntegerMatrix::from_owned_columns({:?}) = {}",
            vs,
            IntegerMatrix::from_owned_columns(vs.clone())
        );
    }
}

fn benchmark_integer_matrix_from_columns_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix::from_columns(&[IntegerVector])",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen()
            .get(gm, config)
            .map(|m| (columns_of(&m), m)),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(_, m)| {
                usize::exact_from(
                    m.elements_ref()
                        .iter()
                        .map(SignificantBits::significant_bits)
                        .sum::<u64>(),
                )
            },
            bucketing_label: "m's total element bits".to_string(),
        },
        &mut [
            (
                "IntegerMatrix::from_columns(&[IntegerVector])",
                &mut |(vs, _)| {
                    no_out!(IntegerMatrix::from_columns(&vs));
                },
            ),
            (
                "IntegerMatrix::from_owned_columns(Vec<IntegerVector>)",
                &mut |(vs, _)| {
                    no_out!(IntegerMatrix::from_owned_columns(vs));
                },
            ),
        ],
    );
}
