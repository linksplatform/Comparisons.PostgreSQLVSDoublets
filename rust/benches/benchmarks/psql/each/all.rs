//! # PostgreSQL Each All Benchmark
//!
//! This benchmark measures the performance of querying all links in PostgreSQL.
//!
//! ## Implementation
//!
//! SQL query:
//! ```sql
//! SELECT id, source, target FROM links
//! ```
//!
//! - Full table scan
//! - Time complexity: O(n) where n is the number of links + network overhead

use std::time::{Duration, Instant};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{data::Flow, Doublets};
use linkspsql::{bench, connect, Benched, Client, Exclusive, Fork, Transaction};

use crate::tri;

/// Runs the each_all benchmark on a PostgreSQL backend.
fn bench<B: Benched + Doublets<usize>>(
    group: &mut BenchmarkGroup<WallTime>,
    id: &str,
    mut benched: B,
) {
    let handler = |_| Flow::Continue;
    group.bench_function(id, |bencher| {
        bench!(|fork| as B {
            let _ = elapsed! { fork.each(handler) };
        })(bencher, &mut benched);
    });
}

/// Creates benchmark for PostgreSQL backends on querying all links.
pub fn each_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("Each_All");

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
