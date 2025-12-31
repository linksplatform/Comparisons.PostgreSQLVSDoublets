//! # Doublets Each All Benchmark
//!
//! This benchmark measures the performance of querying all links in Doublets.
//!
//! ## Implementation
//!
//! Query pattern: `[*, *, *]` - matches all links
//! - Sequential iteration through the internal array
//! - Time complexity: O(n) where n is the number of links

use std::{
    alloc::Global,
    time::{Duration, Instant},
};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{
    data::Flow,
    mem::{Alloc, FileMapped},
    parts::LinkPart,
    split::{self, DataPart, IndexPart},
    unit, Doublets,
};
use linkspsql::{bench, Benched, Fork};

use crate::tri;

/// Runs the each_all benchmark on a Doublets backend.
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

/// Creates benchmark for Doublets backends on querying all links.
pub fn each_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("Each_All");

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
