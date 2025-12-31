//! # Doublets Update Links Benchmark
//!
//! This benchmark measures the performance of updating links in Doublets.
//!
//! ## Implementation
//!
//! Doublets updates links by:
//! - Modifying source and target values in storage
//! - Re-indexing if source or target changed
//! - Time complexity: O(log n) for index updates

use std::{
    alloc::Global,
    time::{Duration, Instant},
};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{
    mem::{Alloc, FileMapped},
    split::{self, DataPart, IndexPart},
    unit::{self, LinkPart},
    Doublets,
};
use linkspsql::{background_links, bench, benchmark_links, Benched, Fork};

use crate::tri;

/// Runs the update benchmark on a Doublets backend.
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

/// Creates benchmark for Doublets backends on link updates.
pub fn update_links(c: &mut Criterion) {
    let mut group = c.benchmark_group("Update");

    tri! {
        bench(
            &mut group,
            "Doublets_United_Volatile",
            unit::Store::<usize, Alloc<LinkPart<_>, Global>>::setup(()).unwrap()
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
            split::Store::<usize, Alloc<DataPart<_>, _>, Alloc<IndexPart<_>, _>>::setup(()).unwrap()
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
