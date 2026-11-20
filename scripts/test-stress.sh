#!/bin/sh
# Full Mithqalite stress run, including the ignored huge-pi tests.
set -eu

threads="${TEST_THREADS:-4}"
jobs="${CARGO_BUILD_JOBS:-4}"
cases="${MITHQALITE_TEST_CASES:-10000}"

cd "$(dirname "$0")/.."
MITHQALITE_TEST_CASES="$cases" CARGO_BUILD_JOBS="$jobs" \
    cargo test --release -p mithqalite-float --features test_build -- \
    --test-threads "$threads" --include-ignored
