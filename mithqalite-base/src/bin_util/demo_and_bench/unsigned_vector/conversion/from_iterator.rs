// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use mithqalite_base::strings::ToDebugString;
use mithqalite_base::test_util::bench::bucketers::vec_len_bucketer;
use mithqalite_base::test_util::bench::{BenchmarkType, run_benchmark};
use mithqalite_base::test_util::generators::common::{GenConfig, GenMode};
use mithqalite_base::test_util::generators::unsigned_vec_gen;
use mithqalite_base::test_util::runner::Runner;
use mithqalite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_from_iter);
    register_bench!(runner, benchmark_unsigned_vector_from_iter);
}

fn demo_unsigned_vector_from_iter(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "UnsignedVector<u64>::from_iter({}) = {}",
            xs.to_debug_string(),
            xs.iter().copied().collect::<UnsignedVector<u64>>()
        );
    }
}

fn benchmark_unsigned_vector_from_iter(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>::from_iter(I)",
        BenchmarkType::Single,
        unsigned_vec_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| {
            no_out!(xs.into_iter().collect::<UnsignedVector<u64>>());
        })],
    );
}
