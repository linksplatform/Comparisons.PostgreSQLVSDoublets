#![feature(allocator_api)]

use {
    criterion::Criterion,
    doublets::{
        mem::{Alloc, FileMapped},
        split, unit, Doublets,
    },
    linkspsql::{benchmark, connect, elapsed, prepare_file, SetupTeardown, BACKGROUND_LINKS},
    std::{
        error::Error,
        time::{Duration, Instant},
    },
    tokio::runtime::Runtime,
};

pub fn create_links(c: &mut Criterion) -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new()?;
    let mut group = c.benchmark_group("Create");
    let allocator = std::alloc::Global;
    let united = prepare_file("united.links")?;
    let split_data = prepare_file("split_data.links")?;
    let split_index = prepare_file("split_index.links")?;
    {
        let mut client = runtime.block_on(connect())?;
        let mut background = BACKGROUND_LINKS;
        let _ = client.setup();
        benchmark! {
            group,
            "PSQL_NonTransaction",
            {
                for _ in 0..1_000 {
                    let _ = client.create_point();
                }
            },
            {
                for index in (background + 1..=background + 1_000).rev() {
                    let _ = client.delete(index);
                }
                background += 1_000;
            }
        }
        let _ = client.teardown();
    }
    {
        let mut client = runtime.block_on(connect())?;
        let mut background = BACKGROUND_LINKS;
        let mut transaction = client.transaction()?;
        let _ = transaction.setup();
        let _ = transaction.commit();
        benchmark! {
            group,
            "PSQL_Transaction",
            let mut transaction = client.transaction().unwrap(),
            {
                for _ in 0..1_000 {
                    let _ = transaction.create_point();
                }
                let _ = transaction.commit();
            },
            {
                let mut transaction = client.transaction().unwrap();
                for index in (background + 1..=background + 1_000).rev() {
                    let _ = transaction.delete(index);
                }
                let _ = transaction.commit();
                background += 1_000;
            }
        }
        let _ = client.transaction()?.teardown();
    }
    {
        let storage = Alloc::new(allocator);
        let mut links = unit::Store::<usize, _>::new(storage)?;
        let _ = links.setup();
        benchmark! {
            group,
            "Doublets_United_Volatile",
            {
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            },
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS + 1_000).rev() {
                    let _ = links.delete(index);
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
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            },
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS + 1_000).rev() {
                    let _ = links.delete(index);
                }
            }
        }
        let _ = links.teardown();
    }
    {
        let data = Alloc::new(allocator);
        let index = Alloc::new(allocator);
        let mut links = split::Store::<usize, _, _>::new(data, index)?;
        let _ = links.setup();
        benchmark! {
            group,
            "Doublets_Split_Volatile",
            {
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            },
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS + 1_000).rev() {
                    let _ = links.delete(index);
                }
            }
        }
        let _ = links.teardown();
    }
    {
        let split_data = FileMapped::new(split_data)?;
        let split_index = FileMapped::new(split_index)?;
        let mut links = split::Store::<usize, _, _>::new(split_data, split_index)?;
        let _ = links.setup();
        benchmark! {
            group,
            "Doublets_Split_NonVolatile",
            {
                for _ in 0..1_000 {
                    let _ = links.create_point();
                }
            },
            {
                for index in (BACKGROUND_LINKS + 1..=BACKGROUND_LINKS + 1_000).rev() {
                    let _ = links.delete(index);
                }
            }
        }
        let _ = links.teardown();
    }
    group.finish();
    Ok(())
}
