#!/bin/sh
# Routine Mithqalite nz gate. Large multiplication stress tests are ignored by default.
set -eu

threads="${TEST_THREADS:-4}"
jobs="${CARGO_BUILD_JOBS:-4}"

cd "$(dirname "$0")/.."
CARGO_BUILD_JOBS="$jobs" cargo test --release -p mithqalite-nz --features test_build -- \
    --test-threads "$threads"
