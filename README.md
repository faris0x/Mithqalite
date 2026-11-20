# Mithqalite

Mithqalite is an independent fork of [Malachite](https://github.com/mhogrefe/malachite), the arbitrary-precision arithmetic library created by Mikhail Hogrefe. It is the numeric core of [Mithqal](https://github.com/faris0x/Mithqal).

Mithqalite does not follow Malachite upstream. It has its own direction, release cadence, and compatibility decisions. Changes made here are not expected to match Malachite, and changes made in Malachite are not expected to be merged here.

## Scope

Mithqalite owns the numeric internals that Mithqal depends on:

- Arbitrary-precision integers and rationals.
- Arbitrary-precision binary floats with correct rounding.
- Elementary functions, constants, and rounding modes.
- Internal performance work that requires access to the numeric representation.

Mithqal owns the engine-facing numeric types, complex arithmetic, hypercomplex types, fixed-point arithmetic, special functions, formatting, and interning.

## Crates

The repository is a Cargo workspace:

- `mithqalite` re-exports the public numeric types.
- `mithqalite-base` provides shared numeric traits and utilities.
- `mithqalite-nz` provides natural and integer arithmetic.
- `mithqalite-q` provides rational arithmetic.
- `mithqalite-float` provides binary floating-point arithmetic.

The crate names are retained from Malachite for source and dependency compatibility. The project name for this repository is Mithqalite.

## Building and testing

```text
cargo build --release
cargo test --release --workspace
```

The float tests use the `test_build` feature:

```text
cargo test --release -p mithqalite-float --features test_build
```

The routine gate caps randomized property generators at 256 cases and skips the
huge-pi and heavy-precision stress tests:

```text
scripts/test-gate.sh
```

The full stress run uses 10,000 property cases and includes the ignored huge-pi
tests:

```text
scripts/test-stress.sh
```

The nz crate has its own gate and stress scripts. The nz gate skips the ignored
large-multiplication stress test:

```text
scripts/test-nz-gate.sh
scripts/test-nz-stress.sh
```

Set `MITHQALITE_TEST_CASES` to override the property case count, and
`TEST_THREADS` to override the test thread count.

## Test suite status and confidence

The complete float suite is expensive. On a four-core machine the library
tests alone took 5 hours 49 minutes, dominated by the huge-pi and
heavy-precision cases. That is not acceptable as a routine gate.

The routine float gate runs 4,655 tests in about 3.5 minutes with a warm build,
plus about 3 seconds for doctests. It ignores 58 stress tests. The nz gate runs
6,345 tests in about 2.5 minutes and ignores one large-multiplication stress
test.

Float coverage is 92.13% of lines and 93.60% of branches after excluding test
utilities. It reaches 95.04% of lines and 96.96% of branches after also
excluding three modules that contain only by-value and by-reference delegation
wrappers for `mul_add_mul`, `mul_sub_mul`, and `rem`. Their shared arithmetic
implementations are covered through the non-wrapper tests.

nz coverage has not been measured yet. The full nz coverage run was interrupted
before producing results, so confidence in nz is not quantitatively guaranteed.
A gentler measurement focused on the core natural and integer modules used by
Mithqal is in progress.

The numeric crates also build without the standard library:

```text
cargo check -p mithqalite-float --lib --no-default-features
cargo check -p mithqalite --lib --no-default-features --features naturals_and_integers,rationals,floats
```

These builds use `core` and `alloc`, and the final binary must provide a global allocator.

## License

Mithqalite remains free software under the GNU Lesser General Public License, version 3.0 only. See [COPYING](COPYING) and [COPYING.LESSER](COPYING.LESSER).

Original Malachite code is copyright © Mikhail Hogrefe and the Malachite contributors. Mithqalite modifications are copyright © Faris Alfarhan and the Mithqalite contributors.

Mithqal is licensed under the GNU GPL version 3.0 only.
