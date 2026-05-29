//! # PostgreSQL Backend Implementations
//!
//! This module contains [`Benched`] trait implementations for PostgreSQL
//! storage variants.
//!
//! ## Storage Backends
//!
//! | Backend                    | Description                         |
//! |----------------------------|-------------------------------------|
//! | `Exclusive<Client<T>>`     | Direct SQL execution (no transaction) |
//! | `Exclusive<Transaction<T>>`| SQL execution within transaction    |
//!
//! ## Table Management
//!
//! Each backend's lifecycle manages a `links` table:
//! - `fork()` calls `CREATE TABLE` to set up the table
//! - `unfork()` calls `DROP TABLE` to clean up
//!
//! ## SQL Schema
//!
//! The `links` table has the following structure:
//! ```sql
//! CREATE TABLE links (
//!     id BIGINT PRIMARY KEY,
//!     source BIGINT NOT NULL,
//!     target BIGINT NOT NULL
//! )
//! ```

use crate::{Client, Exclusive, Fork, Result, Sql, Transaction};
use doublets::data::LinkReference;

use super::Benched;

/// Benched implementation for PostgreSQL client (non-transactional).
impl<T: LinkReference> Benched for Exclusive<Client<T>> {
    type Builder<'a> = ();

    fn setup(_: Self::Builder<'_>) -> Result<Self> {
        // Safety: Exclusive wrapper ensures single-threaded access
        unsafe { Ok(Exclusive::new(crate::connect()?)) }
    }

    fn fork(&mut self) -> Fork<'_, Self> {
        let _ = self.create_table();
        Fork(self)
    }

    unsafe fn unfork(&mut self) {
        let _ = self.drop_table();
    }
}

/// Benched implementation for PostgreSQL transaction.
impl<'a, T: LinkReference> Benched for Exclusive<Transaction<'a, T>> {
    type Builder<'b> = &'a mut Client<T>;

    fn setup(builder: Self::Builder<'_>) -> Result<Self> {
        let mut transaction = builder.transaction()?;
        transaction.create_table()?;
        // Safety: Exclusive wrapper ensures single-threaded access
        unsafe { Ok(Exclusive::new(transaction)) }
    }

    fn fork(&mut self) -> Fork<'_, Self> {
        let _ = self.create_table();
        Fork(self)
    }

    unsafe fn unfork(&mut self) {
        let _ = self.drop_table();
    }
}
