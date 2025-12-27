use {
    crate::tri,
    criterion::{BenchmarkGroup, Criterion, measurement::WallTime},
    doublets::{
        Doublets,
        mem::{Alloc, FileMapped}, split, split::{DataPart, IndexPart},
        unit, unit::LinkPart,
    },
    linkspsql::{background_links, benchmark_links, bench, Benched, Transaction, Exclusive, connect, Client, Fork},
    std::{alloc::Global, time::{Duration, Instant}},
};

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
