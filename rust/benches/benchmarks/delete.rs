use {
    criterion::Criterion,
    doublets::{
        mem::{Alloc, FileMapped},
        split, unit, Doublets,
    },
    linkspsql::{benchmark, connect, elapsed, prepare_file, Benched, BACKGROUND_LINKS},
    std::{
        alloc::Global,
        error::Error,
        time::{Duration, Instant},
    },
    tokio::runtime::Runtime,
};

pub fn delete_links(c: &mut Criterion) -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new()?;
    let mut group = c.benchmark_group("Delete");
    let united = prepare_file("united.links")?;
    let split_data = prepare_file("split_data.links")?;
    let split_index = prepare_file("split_index.links")?;
    {
        let mut client = runtime.block_on(connect())?;
        let mut background = BACKGROUND_LINKS - 1_000;
        let _ = client.setup();
        benchmark! {
            group,
            "PSQL_NonTransaction",
            {
                for index in (background + 1..=background + 1_000).rev() {
                    let _ = client.delete(index);
                }
            },
            above {
                for _ in 0..1_000 {
                    let _ = client.create_point();
                }
                background += 1_000;
            }
        }
        let _ = client.teardown();
    }
    {
        let mut client = runtime.block_on(connect())?;
        let mut transaction = client.transaction()?;
        let mut background = BACKGROUND_LINKS - 1_000;
        let _ = transaction.setup();
        let _ = transaction.commit();
        benchmark! {
            group,
            "PSQL_Transaction",
            let mut transaction = client.transaction().unwrap(),
            {
                for index in (background + 1..=background + 1_000).rev() {
                    let _ = transaction.delete(index);
                }
                let _ = transaction.commit();
            },
            above {
                let mut transaction = client.transaction().unwrap();
                for _ in 0..1_000 {
                    let _ = transaction.create_point();
                }
                let _ = transaction.commit();
                background += 1_000;
            }
        }
        let _ = client.transaction()?.teardown();
    }
    {
        let storage = Alloc::new(Global);
        let mut links = unit::Store::<usize, _>::new(storage)?;
        let _ = links.setup();
        benchmark! {
            group,
            "Doublets_United_Volatile",
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS+1_000).rev() {
                    let _ = links.delete(index);
                }
            },
            above {
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            }
        }
        let _ = links.teardown();
    }
    {
        let storage = FileMapped::new(united)?;
        let mut links = unit::Store::<usize, _>::new(storage)?;
        let _ = links.setup();
        benchmark! {
            group,
            "Doublets_United_NonVolatile",
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS+1_000).rev() {
                    let _ = links.delete(index);
                }
            },
            above {
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            }
        }
        let _ = links.teardown();
    }
    {
        let data = Alloc::new(Global);
        let index = Alloc::new(Global);
        let mut links = split::Store::<usize, _, _>::new(data, index)?;
        let _ = links.setup();
        benchmark! {
            group,
            "Doublets_Split_Volatile",
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS+1_000).rev() {
                    let _ = links.delete(index);
                }
            },
            above {
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            }
        }
        let _ = links.teardown();
    }
    {
        let split_data = FileMapped::new(split_data)?;
        let split_index = FileMapped::new(split_index)?;
        let mut links = split::Store::<usize, _, _>::new(split_data, split_index).unwrap();
        let _ = links.setup();
        benchmark! {
            group,
            "Doublets_Split_NonVolatile",
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS+1_000).rev() {
                    let _ = links.delete(index);
                }
            },
            above {
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            }
        }
        let _ = links.teardown();
    }
    group.finish();
    Ok(())
}
