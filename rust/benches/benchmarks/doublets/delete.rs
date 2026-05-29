//! # Doublets Delete Links Benchmark
//!
//! This benchmark measures the performance of deleting links in Doublets.
//!
//! ## Implementation
//!
//! Doublets deletes links by:
//! - Removing the link from source and target indexes
//! - Marking the slot as free for reuse
//! - Time complexity: O(log n) for index updates

use std::time::{Duration, Instant};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{
    mem::{FileMapped, Global},
    split::{self, DataPart, IndexPart},
    unit::{self, LinkPart},
    Doublets,
};
use linkspsql::{background_links, bench, benchmark_links, Benched, Fork};

use crate::tri;

/// Runs the delete benchmark on a Doublets backend.
fn bench<B: Benched + Doublets<usize>>(
    group: &mut BenchmarkGroup<WallTime>,
    id: &str,
    mut benched: B,
) {
    let bg_links = background_links();
    let links = benchmark_links();
    group.bench_function(id, |bencher| {
        bench!(|fork| as B {
            for _prepare in bg_links..bg_links + links {
                let _ = fork.create_point();
            }
            for id in (bg_links..=bg_links + links).rev() {
                let _ = elapsed! {fork.delete(id)?};
            }
        })(bencher, &mut benched);
    });
}

/// Creates benchmark for Doublets backends on link deletion.
pub fn delete_links(c: &mut Criterion) {
    let mut group = c.benchmark_group("Delete");

    tri! {
        bench(
            &mut group,
            "Doublets_United_Volatile",
            unit::Store::<usize, Global<LinkPart<_>>>::setup(()).unwrap()
        )
    }
    tri! {
        bench(
            &mut group,
            "Doublets_United_NonVolatile",
            unit::Store::<usize, FileMapped<LinkPart<_>>>::setup("united.links").unwrap()
        )
    }
    tri! {
        bench(
            &mut group,
            "Doublets_Split_Volatile",
            split::Store::<usize, Global<DataPart<_>>, Global<IndexPart<_>>>::setup(()).unwrap()
        )
    }
    tri! {
        bench(
            &mut group,
            "Doublets_Split_NonVolatile",
            split::Store::<usize, FileMapped<_>, FileMapped<_>>::setup(("split_index.links", "split_data.links")).unwrap()
        )
    }

    group.finish();
}
