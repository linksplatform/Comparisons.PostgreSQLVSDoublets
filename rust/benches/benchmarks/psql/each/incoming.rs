//! # PostgreSQL Each Incoming Benchmark
//!
//! This benchmark measures the performance of querying links by target in PostgreSQL.
//!
//! ## Implementation
//!
//! SQL query:
//! ```sql
//! SELECT id, source, target FROM links WHERE target = $1
//! ```
//!
//! - Target column index lookup
//! - Time complexity: O(log n + k) where k is number of matching links + network overhead

use std::time::{Duration, Instant};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{
    data::{Flow, LinksConstants},
    Doublets,
};
use linkspsql::{background_links, bench, connect, Benched, Client, Exclusive, Fork, Transaction};

use crate::tri;

/// Runs the each_incoming benchmark on a PostgreSQL backend.
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
                elapsed! {fork.each_by([any, any, index], handler)};
            }
        })(bencher, &mut benched);
    });
}

/// Creates benchmark for PostgreSQL backends on querying links by target.
pub fn each_incoming(c: &mut Criterion) {
    let mut group = c.benchmark_group("Each_Incoming");

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
