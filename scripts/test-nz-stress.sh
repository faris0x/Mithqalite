#!/bin/sh
# Full Mithqalite nz stress run, including the ignored large-multiplication tests.
set -eu

threads="${TEST_THREADS:-4}"
jobs="${CARGO_BUILD_JOBS:-4}"
cases="${MITHQALITE_TEST_CASES:-10000}"

cd "$(dirname "$0")/.."
MITHQALITE_TEST_CASES="$cases" CARGO_BUILD_JOBS="$jobs" \
    cargo test --release -p mithqalite-nz --features test_build -- \
    --test-threads "$threads" --include-ignored
