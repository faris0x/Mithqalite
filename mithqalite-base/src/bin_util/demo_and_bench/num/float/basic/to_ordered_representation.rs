// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use mithqalite_base::num::basic::floats::PrimitiveFloat;
use mithqalite_base::num::float::NiceFloat;
use mithqalite_base::test_util::bench::bucketers::primitive_float_bucketer;
use mithqalite_base::test_util::bench::{BenchmarkType, run_benchmark};
use mithqalite_base::test_util::generators::common::{GenConfig, GenMode};
use mithqalite_base::test_util::generators::primitive_float_gen_var_11;
use mithqalite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_to_ordered_representation);
    register_primitive_float_benches!(runner, benchmark_to_ordered_representation);
}

fn demo_to_ordered_representation<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_11::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_ordered_representation({}) = {}",
            NiceFloat(x),
            x.to_ordered_representation()
        );
    }
}

fn benchmark_to_ordered_representation<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_ordered_representation()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_11::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |x| no_out!(x.to_ordered_representation()))],
    );
}
