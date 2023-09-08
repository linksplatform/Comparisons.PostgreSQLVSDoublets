use {
    linkspsql::{connect, Sql},
    criterion::{measurement::WallTime, BenchmarkGroup, Criterion},
    doublets::{data::LinkType, mem::Alloc, split, unit, Doublets},
    linkspsql::{
        benchmark, elapsed, prepare_file, Benched, Client, Fork, Result, Transaction,
        BACKGROUND_LINKS,
    },
    std::{
        alloc::Global,
        time::{Duration, Instant},
    },
    tokio::runtime::{Builder, Runtime},
};

fn bench<S: Benched + Doublets<T>, T: LinkType>(
    builder: S::Builder<'_>,
    id: &str,
    group: &mut BenchmarkGroup<WallTime>,
) -> Result<()> {
    let mut storage = S::setup(builder)?;
    group.bench_function(id, |b| {
        benchmark! {
            b,
            let mut fork = storage.fork();
            for _ in 0..1_000 {
                let _ = fork.create_point();
            }
        }
    });
    Ok(())
}

pub fn create_links(c: &mut Criterion) -> Result<()> {
    let mut group = c.benchmark_group("Create");
    //let united = prepare_file("united.links")?;
    //let split_data = prepare_file("split_data.links")?;
    //let split_index = prepare_file("split_index.links")?;
    //bench::<Client<usize>, _>(&runtime, "PSQL_NonTransaction", &mut group).unwrap();
    {
        let runtime = Runtime::new().unwrap();
        let mut client = runtime.block_on(connect()).unwrap();
        bench::<Transaction<'_, usize>, _>(&mut client, "PSQL_Transaction", &mut group).unwrap();
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
    group.finish();
    Ok(())
}
