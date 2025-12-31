//! # Doublets Each Incoming Benchmark
//!
//! This benchmark measures the performance of querying links by target in Doublets.
//!
//! ## Implementation
//!
//! Query pattern: `[*, *, tgt]` - matches links by target
//! - Target index tree traversal
//! - Time complexity: O(log n + k) where k is the number of matching links

use std::{
    alloc::Global,
    time::{Duration, Instant},
};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{
    data::{Flow, LinksConstants},
    mem::{Alloc, FileMapped},
    parts::LinkPart,
    split::{self, DataPart, IndexPart},
    unit, Doublets,
};
use linkspsql::{background_links, bench, Benched, Fork};

use crate::tri;

/// Runs the each_incoming benchmark on a Doublets backend.
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

/// Creates benchmark for Doublets backends on querying links by target.
pub fn each_incoming(c: &mut Criterion) {
    let mut group = c.benchmark_group("Each_Incoming");

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
