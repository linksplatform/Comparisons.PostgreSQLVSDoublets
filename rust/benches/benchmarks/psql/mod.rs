//! # PostgreSQL Benchmark Implementations
//!
//! This module contains all benchmark implementations for PostgreSQL.
//! Each benchmark tests a specific database operation using SQL queries
//! executed via libpq client.
//!
//! ## Benchmarked Operations
//!
//! | Benchmark       | SQL Query                                                   |
//! |-----------------|-------------------------------------------------------------|
//! | `create_links`  | `INSERT INTO links (id, source, target) VALUES ($1, $1, $1)`|
//! | `delete_links`  | `DELETE FROM links WHERE id = $1`                           |
//! | `update_links`  | `UPDATE links SET source = $2, target = $3 WHERE id = $1`   |
//! | `each_all`      | `SELECT id, source, target FROM links`                      |
//! | `each_identity` | `SELECT id, source, target FROM links WHERE id = $1`        |
//! | `each_concrete` | `SELECT ... WHERE source = $1 AND target = $2`              |
//! | `each_outgoing` | `SELECT ... WHERE source = $1`                              |
//! | `each_incoming` | `SELECT ... WHERE target = $1`                              |
//!
//! ## Storage Backends Tested
//!
//! - `PSQL_NonTransaction` - Direct SQL execution without transaction wrapping
//! - `PSQL_Transaction` - SQL execution within explicit transaction

mod create;
mod delete;
pub mod each;
mod update;

pub use create::create_links;
pub use delete::delete_links;
pub use each::*;
pub use update::update_links;
