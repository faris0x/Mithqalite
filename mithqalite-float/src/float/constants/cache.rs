// Copyright © 2026 Faris Alfarhan
//
// This file is part of Mithqalite.
//
// Mithqalite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License as published by the Free Software Foundation; either version 3 of
// the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! A small multi-entry cache for elementary constants.
//!
//! The cache is available when the `std` feature is enabled. It keeps the most recent
//! `CACHE_CAPACITY` rounded values per constant, keyed by precision and rounding mode. This covers
//! repeated constant requests made by the engine and by argument reduction without letting a
//! single constant grow without bound. Builds without `std` use the uncached implementation.

#[cfg(feature = "std")]
use crate::Float;
#[cfg(feature = "std")]
use core::cmp::Ordering;
#[cfg(feature = "std")]
use mithqalite_base::rounding_modes::RoundingMode;
#[cfg(feature = "std")]
use std::sync::{Mutex, PoisonError};

#[cfg(feature = "std")]
const CACHE_CAPACITY: usize = 8;

#[cfg(feature = "std")]
struct Entry {
    prec: u64,
    rounding: RoundingMode,
    value: Float,
    ordering: Ordering,
}

#[cfg(feature = "std")]
pub(crate) struct ConstantCache {
    entries: Mutex<alloc::vec::Vec<Entry>>,
}

#[cfg(feature = "std")]
impl ConstantCache {
    pub(crate) const fn new() -> Self {
        Self {
            entries: Mutex::new(alloc::vec::Vec::new()),
        }
    }

    pub(crate) fn get(&self, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
        let entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        entries
            .iter()
            .find(|entry| entry.prec == prec && entry.rounding == rm)
            .map(|entry| (entry.value.clone(), entry.ordering))
    }

    pub(crate) fn put(&self, prec: u64, rm: RoundingMode, value: Float, ordering: Ordering) {
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.prec == prec && entry.rounding == rm)
        {
            entry.value = value;
            entry.ordering = ordering;
            return;
        }
        if entries.len() == CACHE_CAPACITY {
            entries.remove(0);
        }
        entries.push(Entry {
            prec,
            rounding: rm,
            value,
            ordering,
        });
    }
}

#[cfg(not(feature = "std"))]
pub(crate) struct ConstantCache;

#[cfg(not(feature = "std"))]
impl ConstantCache {
    pub(crate) const fn new() -> Self {
        Self
    }

    pub(crate) fn get(
        &self,
        _prec: u64,
        _rm: mithqalite_base::rounding_modes::RoundingMode,
    ) -> Option<(crate::Float, core::cmp::Ordering)> {
        None
    }

    pub(crate) fn put(
        &self,
        _prec: u64,
        _rm: mithqalite_base::rounding_modes::RoundingMode,
        _value: crate::Float,
        _ordering: core::cmp::Ordering,
    ) {
    }
}
