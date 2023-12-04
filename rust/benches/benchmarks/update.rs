use {
    crate::tri,
    criterion::{measurement::WallTime, BenchmarkGroup, Criterion},
    doublets::{
        mem::{Alloc, FileMapped},
        split::{self, DataPart, IndexPart},
        unit::{self, LinkPart},
        Doublets,
    },
    linkspsql::{bench, connect, Benched, Client, Exclusive, Fork, Transaction, BACKGROUND_LINKS},
    std::{
        alloc::Global,
        time::{Duration, Instant},
    },
};

fn bench<B: Benched + Doublets<usize>>(
    group: &mut BenchmarkGroup<WallTime>,
    id: &str,
    mut benched: B,
) {
    group.bench_function(id, |bencher| {
        bench!(|fork| as B {
            for id in BACKGROUND_LINKS - 999..=BACKGROUND_LINKS {
                let _ = elapsed! {fork.update(id, 0, 0)?};
                let _ = elapsed! {fork.update(id, id, id)?};
            }
        })(bencher, &mut benched);
    });
}

pub fn update_links(c: &mut Criterion) {
    let mut group = c.benchmark_group("Update");
    tri! {
        bench(&mut group, "PSQL_NonTransaction", Exclusive::<Client<usize>>::setup(())?);
        let mut client = connect()?;
        bench(
            &mut group,
            "PSQL_Transaction",
            Exclusive::<Transaction<'_, usize>>::setup(&mut client)?,
        );
        bench(
            &mut group,
            "Doublets_United_Volatile",
            unit::Store::<usize, Alloc<LinkPart<_>, Global>>::setup(())?
        )
        bench(
            &mut group,
            "Doublets_United_NonVolatile",
            unit::Store::<usize, FileMapped<LinkPart<_>>>::setup("united.links")?
        )
        bench(
            &mut group,
            "Doublets_Split_Volatile",
            split::Store::<usize, Alloc<DataPart<_>, _>, Alloc<IndexPart<_>, _>>::setup(())?
        )
        bench(
            &mut group,
            "Doublets_Split_NonVolatile",
            split::Store::<usize, FileMapped<_>, FileMapped<_>>::setup(("split_index.links", "split_data.links"))?
        )
    }
    group.finish();
}
