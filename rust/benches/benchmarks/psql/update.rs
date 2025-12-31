//! # PostgreSQL Update Links Benchmark
//!
//! This benchmark measures the performance of updating links in PostgreSQL.
//!
//! ## Implementation
//!
//! PostgreSQL executes this SQL query:
//! ```sql
//! UPDATE links SET source = $2, target = $3 WHERE id = $1
//! ```
//!
//! - Makes connection to PostgreSQL database
//! - Executes UPDATE statement
//! - Time complexity: O(log n) for B-tree index updates + network overhead

use std::time::{Duration, Instant};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::Doublets;
use linkspsql::{background_links, bench, benchmark_links, connect, Benched, Client, Exclusive, Fork, Transaction};

use crate::tri;

/// Runs the update benchmark on a PostgreSQL backend.
fn bench<B: Benched + Doublets<usize>>(
    group: &mut BenchmarkGroup<WallTime>,
    id: &str,
    mut benched: B,
) {
    let bg_links = background_links();
    let links = benchmark_links();
    group.bench_function(id, |bencher| {
        bench!(|fork| as B {
            for id in bg_links - (links - 1)..=bg_links {
                let _ = elapsed! {fork.update(id, 0, 0)?};
                let _ = elapsed! {fork.update(id, id, id)?};
            }
        })(bencher, &mut benched);
    });
}

/// Creates benchmark for PostgreSQL backends on link updates.
pub fn update_links(c: &mut Criterion) {
    let mut group = c.benchmark_group("Update");

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
