//! # Doublets Query Benchmarks
//!
//! This module contains query (each) benchmarks for Doublets backends.
//!
//! ## Query Types
//!
//! | Benchmark       | Query Pattern      | Implementation                |
//! |-----------------|--------------------|-------------------------------|
//! | `each_all`      | `[*, *, *]`        | Sequential array iteration    |
//! | `each_identity` | `[id, *, *]`       | Direct array access           |
//! | `each_concrete` | `[*, src, tgt]`    | Index tree lookup + filter    |
//! | `each_outgoing` | `[*, src, *]`      | Source index tree traversal   |
//! | `each_incoming` | `[*, *, tgt]`      | Target index tree traversal   |

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
