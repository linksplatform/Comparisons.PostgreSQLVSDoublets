use {
    crate::tri,
    criterion::{measurement::WallTime, BenchmarkGroup, Criterion},
    doublets::{
        data::{Flow, LinkType},
        mem::{Alloc, FileMapped},
        parts::LinkPart,
        split::{self, DataPart, IndexPart},
        unit, Doublets,
    },
    linkspsql::{bench, connect, Benched, Client, Exclusive, Fork, Transaction},
    std::{
        alloc::Global,
        time::{Duration, Instant},
    },
};

fn bench<T: LinkType, B: Benched + Doublets<T>>(
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
