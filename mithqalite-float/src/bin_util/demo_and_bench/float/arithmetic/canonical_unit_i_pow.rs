// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use mithqalite_base::num::arithmetic::traits::CanonicalUnitIPow;
use mithqalite_base::test_util::bench::{BenchmarkType, run_benchmark};
use mithqalite_base::test_util::generators::common::{GenConfig, GenMode};
use mithqalite_base::test_util::runner::Runner;
use mithqalite_float::test_util::bench::bucketers::float_complexity_bucketer;
use mithqalite_float::test_util::generators::float_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_canonical_unit_i_pow);
    register_bench!(runner, benchmark_float_canonical_unit_i_pow);
}

fn demo_float_canonical_unit_i_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "({x}).canonical_unit_i_pow() = {}",
            x.canonical_unit_i_pow()
        );
    }
}

fn benchmark_float_canonical_unit_i_pow(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.canonical_unit_i_pow()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.canonical_unit_i_pow()))],
    );
}
