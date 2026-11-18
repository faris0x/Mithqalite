// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Macros that define an item whose visibility widens to `pub` under the `test_build` feature, so
// that the test and benchmark crates can reach it while an ordinary build keeps it hidden.
//
// Each name reads as `<visibility>_test_<kind>`, where the visibility is the one the item has
// *without* `test_build`: `private_*` items become private and `crate_*` items become `pub(crate)`.
// The kind is the item the macro emits, so `crate_test_const_fn` is a `const fn` and
// `crate_test_const` is a `const`.
//
// These are also defined, identically, in the other Malachite crates. That is deliberate: lints
// that report against the span of a macro-generated item -- `clippy::missing_const_for_fn` among
// them -- go silent when the macro comes from a *different* crate, so a crate that imported these
// would lose that coverage over every item they generate.

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! private_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! private_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! crate_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! crate_test_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub(crate) fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! crate_test_struct {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub struct $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! crate_test_struct {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub(crate) struct $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! crate_test_enum {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub enum $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! crate_test_enum {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub(crate) enum $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! private_test_const_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub const fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! private_test_const_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        const fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! crate_test_const_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub const fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! crate_test_const_fn {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub(crate) const fn $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(feature = "test_build")]
#[macro_export]
macro_rules! crate_test_const {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub const $name $( $body )*
    };
}

#[doc(hidden)]
#[cfg(not(feature = "test_build"))]
#[macro_export]
macro_rules! crate_test_const {
    ($( #[$meta:meta] )* $name:ident $( $body:tt )*) => {
        $( #[$meta] )*
        pub(crate) const $name $( $body )*
    };
}
