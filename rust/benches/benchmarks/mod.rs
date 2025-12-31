//! # Benchmark Implementations
//!
//! This module contains all benchmark implementations comparing PostgreSQL and Doublets.
//! Each benchmark tests a specific database operation across all storage backends.
//!
//! ## Module Structure
//!
//! The benchmarks are split into two separate modules for clear comparison:
//!
//! - **[`psql`]** - All PostgreSQL benchmarks using SQL queries via libpq
//! - **[`doublets`]** - All Doublets benchmarks using direct memory access
//!
//! ## Benchmarked Operations
//!
//! | Benchmark       | Operation                                      | What it measures                    |
//! |-----------------|------------------------------------------------|-------------------------------------|
//! | `create_links`  | Insert point links (id = source = target)      | Write performance                   |
//! | `delete_links`  | Remove links by ID                             | Delete performance                  |
//! | `update_links`  | Modify source/target of existing links         | Update performance                  |
//! | `each_all`      | Query all links `[*, *, *]`                    | Full scan performance               |
//! | `each_identity` | Query by ID `[id, *, *]`                       | Primary key lookup                  |
//! | `each_concrete` | Query by source+target `[*, src, tgt]`         | Composite index lookup              |
//! | `each_outgoing` | Query by source `[*, src, *]`                  | Source index lookup                 |
//! | `each_incoming` | Query by target `[*, *, tgt]`                  | Target index lookup                 |
//!
//! ## Storage Backends Tested
//!
//! ### Doublets (4 variants)
//! - `Doublets_United_Volatile` - In-memory unit storage
//! - `Doublets_United_NonVolatile` - File-mapped unit storage
//! - `Doublets_Split_Volatile` - In-memory split storage (separate data/index)
//! - `Doublets_Split_NonVolatile` - File-mapped split storage
//!
//! ### PostgreSQL (2 variants)
//! - `PSQL_NonTransaction` - Direct SQL execution
//! - `PSQL_Transaction` - SQL execution within explicit transaction

pub mod doublets;
pub mod psql;

// Re-export all PostgreSQL benchmarks with psql_ prefix
pub use psql::create_links as psql_create_links;
pub use psql::delete_links as psql_delete_links;
pub use psql::each_all as psql_each_all;
pub use psql::each_concrete as psql_each_concrete;
pub use psql::each_identity as psql_each_identity;
pub use psql::each_incoming as psql_each_incoming;
pub use psql::each_outgoing as psql_each_outgoing;
pub use psql::update_links as psql_update_links;

// Re-export all Doublets benchmarks with doublets_ prefix
pub use self::doublets::create_links as doublets_create_links;
pub use self::doublets::delete_links as doublets_delete_links;
pub use self::doublets::each_all as doublets_each_all;
pub use self::doublets::each_concrete as doublets_each_concrete;
pub use self::doublets::each_identity as doublets_each_identity;
pub use self::doublets::each_incoming as doublets_each_incoming;
pub use self::doublets::each_outgoing as doublets_each_outgoing;
pub use self::doublets::update_links as doublets_update_links;
