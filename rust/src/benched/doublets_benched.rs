//! # Doublets Backend Implementations
//!
//! This module contains [`Benched`] trait implementations for all Doublets
//! storage variants.
//!
//! ## Storage Backends
//!
//! | Backend                          | Description                    |
//! |----------------------------------|--------------------------------|
//! | `unit::Store<T, FileMapped<_>>`  | File-mapped unit storage       |
//! | `unit::Store<T, Global<_>>`      | In-memory unit storage          |
//! | `split::Store<T, FileMapped<_>>` | File-mapped split storage      |
//! | `split::Store<T, Global<_>>`     | In-memory split storage        |
//!
//! ## Cleanup Strategy
//!
//! Each backend's `unfork()` implementation calls `delete_all()` to clean up
//! all links created during the benchmark iteration.

use crate::{map_file, Result};
use doublets::{
    data::LinkReference,
    mem::{FileMapped, Global},
    split::{self, DataPart, IndexPart},
    unit::{self, LinkPart},
    Doublets,
};

use super::Benched;

/// Benched implementation for file-mapped unit storage.
impl<T: LinkReference> Benched for unit::Store<T, FileMapped<LinkPart<T>>> {
    type Builder<'a> = &'a str;

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        Self::new(map_file(builder)?).map_err(Into::into)
    }

    unsafe fn unfork(&mut self) {
        let _ = self.delete_all();
    }
}

/// Benched implementation for in-memory unit storage.
impl<T: LinkReference> Benched for unit::Store<T, Global<LinkPart<T>>> {
    type Builder<'a> = ();

    fn setup(_: Self::Builder<'_>) -> Result<Self> {
        Self::new(Global::new()).map_err(Into::into)
    }

    unsafe fn unfork(&mut self) {
        let _ = self.delete_all();
    }
}

/// Benched implementation for file-mapped split storage.
impl<T: LinkReference> Benched for split::Store<T, FileMapped<DataPart<T>>, FileMapped<IndexPart<T>>> {
    type Builder<'a> = (&'a str, &'a str);

    fn setup((data, index): Self::Builder<'_>) -> Result<Self> {
        Self::new(map_file(data)?, map_file(index)?).map_err(Into::into)
    }

    unsafe fn unfork(&mut self) {
        let _ = self.delete_all();
    }
}

/// Benched implementation for in-memory split storage.
impl<T: LinkReference> Benched
    for split::Store<T, Global<DataPart<T>>, Global<IndexPart<T>>>
{
    type Builder<'a> = ();

    fn setup(_: Self::Builder<'_>) -> Result<Self> {
        Self::new(Global::new(), Global::new()).map_err(Into::into)
    }

    unsafe fn unfork(&mut self) {
        let _ = self.delete_all();
    }
}
