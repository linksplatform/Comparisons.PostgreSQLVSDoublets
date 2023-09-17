use {
    crate::tri,
    criterion::{measurement::WallTime, BenchmarkGroup, Criterion},
    doublets::{data::LinkType, mem::Alloc, split, unit, Doublets},
    linkspsql::{
        bench, connect, elapsed, map_file, Benched, Client, Exclusive, Fork, Result, Sql,
        Transaction, BACKGROUND_LINKS,
    },
    std::{
        alloc::Global,
        time::{Duration, Instant},
    },
    tokio::runtime::{Builder, Runtime},
};

fn bench<T: LinkType, B: Benched + Doublets<T>>(
    group: &mut BenchmarkGroup<WallTime>,
    id: &str,
    mut benched: B,
) {
    group.bench_function(id, |bencher| {
        bench!(|fork| as B {
            for _ in 0..1_000 {
                let _ = fork.create_point()?;
            }
        })(bencher, &mut benched);
    });
}

pub fn create_links(c: &mut Criterion) {
    let mut group = c.benchmark_group("Create");
    //let united = prepare_file("united.links")?;
    //let split_data = prepare_file("split_data.links")?;
    //let split_index = prepare_file("split_index.links")?;
    //bench::<Client<usize>, _>(&runtime, "PSQL_NonTransaction", &mut group).unwrap();
    tri! {
        let mut client = connect().unwrap();
        bench(
            &mut group,
            "PSQL_Transaction",
            Exclusive::<Transaction<'_, usize>>::setup(&mut client).unwrap(),
        );
    }

    /*bench::<unit::Store<usize, Alloc<LinkPart<_>, _>>, _>(
        Global,
        "Doublets_United_Volatile",
        &mut group,
    )
    .unwrap();

    bench::<unit::Store<usize, FileMapped<_>>, _>(
        prepare_file("united.links")?,
        "Doublets_United_NonVolatile",
        &mut group,
    )
    .unwrap();

    bench::<split::Store<usize, Alloc<DataPart<_>, _>, Alloc<IndexPart<_>, _>>, _>(
        Global,
        "Doublets_Split_Volatile",
        &mut group,
    )
    .unwrap();

    bench::<split::Store<usize, FileMapped<_>, FileMapped<_>>, _>(
        (
            prepare_file("split_data.links")?,
            prepare_file("split_index.links")?,
        ),
        "Doublets_Split_NonVolatile",
        &mut group,
    )
    .unwrap();*/
}
