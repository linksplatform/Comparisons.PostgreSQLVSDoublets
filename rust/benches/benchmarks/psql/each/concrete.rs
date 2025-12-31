//! # PostgreSQL Each Concrete Benchmark
//!
//! This benchmark measures the performance of querying links by source+target in PostgreSQL.
//!
//! ## Implementation
//!
//! SQL query:
//! ```sql
//! SELECT id, source, target FROM links WHERE source = $1 AND target = $2
//! ```
//!
//! - Composite index lookup
//! - Time complexity: O(log n) + network overhead

use std::time::{Duration, Instant};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{
    data::{Flow, LinksConstants},
    Doublets,
};
use linkspsql::{background_links, bench, connect, Benched, Client, Exclusive, Fork, Transaction};

use crate::tri;

/// Runs the each_concrete benchmark on a PostgreSQL backend.
fn bench<B: Benched + Doublets<usize>>(
    group: &mut BenchmarkGroup<WallTime>,
    id: &str,
    mut benched: B,
) {
    let handler = |_| Flow::Continue;
    let any = LinksConstants::new().any;
    let bg_links = background_links();
    group.bench_function(id, |bencher| {
        bench!(|fork| as B {
            for index in 1..=bg_links {
                elapsed! {fork.each_by([any, index, index], handler)};
            }
        })(bencher, &mut benched);
    });
}

/// Creates benchmark for PostgreSQL backends on querying links by source+target.
pub fn each_concrete(c: &mut Criterion) {
    let mut group = c.benchmark_group("Each_Concrete");

    tri! {
        bench(&mut group, "PSQL_NonTransaction", Exclusive::<Client<usize>>::setup(()).unwrap());
    }
    tri! {
        let mut client = connect().unwrap();
        bench(
            &mut group,
            "PSQL_Transaction",
            Exclusive::<Transaction<'_, usize>>::setup(&mut client).unwrap(),
        );
    }

    group.finish();
}
