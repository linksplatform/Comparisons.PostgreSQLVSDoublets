//! # PostgreSQL Create Links Benchmark
//!
//! This benchmark measures the performance of creating new links in PostgreSQL.
//!
//! ## Implementation
//!
//! PostgreSQL executes this SQL query:
//! ```sql
//! INSERT INTO links (id, source, target) VALUES ($1, $1, $1)
//! ```
//!
//! - Makes connection to PostgreSQL database
//! - Executes INSERT statement
//! - Time complexity: O(log n) for B-tree index updates + network overhead

use std::time::{Duration, Instant};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::Doublets;
use linkspsql::{bench, benchmark_links, connect, Benched, Client, Exclusive, Fork, Transaction};

use crate::tri;

/// Runs the create benchmark on a PostgreSQL backend.
fn bench<B: Benched + Doublets<usize>>(
    group: &mut BenchmarkGroup<WallTime>,
    id: &str,
    mut benched: B,
) {
    let links = benchmark_links();
    group.bench_function(id, |bencher| {
        bench!(|fork| as B {
            for _ in 0..links {
                let _ = elapsed! {fork.create_point()?};
            }
        })(bencher, &mut benched);
    });
}

/// Creates benchmark for PostgreSQL backends on link creation.
pub fn create_links(c: &mut Criterion) {
    let mut group = c.benchmark_group("Create");

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
