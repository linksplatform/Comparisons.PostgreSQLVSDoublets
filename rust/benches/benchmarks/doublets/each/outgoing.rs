//! # Doublets Each Outgoing Benchmark
//!
//! This benchmark measures the performance of querying links by source in Doublets.
//!
//! ## Implementation
//!
//! Query pattern: `[*, src, *]` - matches links by source
//! - Source index tree traversal
//! - Time complexity: O(log n + k) where k is the number of matching links

use std::time::{Duration, Instant};

use criterion::{measurement::WallTime, BenchmarkGroup, Criterion};
use doublets::{
    data::{Flow, LinksConstants},
    mem::{FileMapped, Global},
    split::{self, DataPart, IndexPart},
    unit::{self, LinkPart},
    Doublets,
};
use linkspsql::{background_links, bench, Benched, Fork};

use crate::tri;

/// Runs the each_outgoing benchmark on a Doublets backend.
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
                let _ = elapsed! {fork.each_by([any, index, any], handler)};
            }
        })(bencher, &mut benched);
    });
}

/// Creates benchmark for Doublets backends on querying links by source.
pub fn each_outgoing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Each_Outgoing");

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
