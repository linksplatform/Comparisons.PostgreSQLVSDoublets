//! # PostgreSQL Query Benchmarks
//!
//! This module contains query (each) benchmarks for PostgreSQL backends.
//!
//! ## Query Types
//!
//! | Benchmark       | SQL Pattern                                   |
//! |-----------------|-----------------------------------------------|
//! | `each_all`      | `SELECT id, source, target FROM links`        |
//! | `each_identity` | `SELECT ... WHERE id = $1`                    |
//! | `each_concrete` | `SELECT ... WHERE source = $1 AND target = $2`|
//! | `each_outgoing` | `SELECT ... WHERE source = $1`                |
//! | `each_incoming` | `SELECT ... WHERE target = $1`                |

mod all;
mod concrete;
mod identity;
mod incoming;
mod outgoing;

pub use all::each_all;
pub use concrete::each_concrete;
pub use identity::each_identity;
pub use incoming::each_incoming;
pub use outgoing::each_outgoing;
