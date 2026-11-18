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

## License

Mithqalite remains free software under the GNU Lesser General Public License, version 3.0 only. See [COPYING](COPYING) and [COPYING.LESSER](COPYING.LESSER).

Original Malachite code is copyright © Mikhail Hogrefe and the Malachite contributors. Mithqalite modifications are copyright © Faris Alfarhan and the Mithqalite contributors.

Mithqal is licensed under the GNU GPL version 3.0 only.
