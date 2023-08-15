use {
    criterion::Criterion,
    doublets::{
        data::{Flow, LinksConstants},
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

pub fn each_identity(c: &mut Criterion) -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new()?;
    let mut group = c.benchmark_group("Each_Identity");
    let allocator = std::alloc::Global;
    let united = prepare_file("united.links")?;
    let split_data = prepare_file("split_data.links")?;
    let split_index = prepare_file("split_index.links")?;
    let handler = |_| Flow::Continue;
    let any = LinksConstants::<usize>::new().any;
    {
        let mut client = runtime.block_on(connect())?;
        let _ = client.setup();
        benchmark! {
            group,
            "PSQL_NonTransaction",
            {
                for index in 1..=1_000 {
                    client.each_by([index, any, any], handler);
                }
                for index in 1_001..=2_000 {
                    client.each_by([index, any, any], handler);
                }
                for index in 2_001..=BACKGROUND_LINKS {
                    client.each_by([index, any, any], handler);
                }
            },
        }
        let _ = client.teardown();
    }
    {
        let mut client = runtime.block_on(connect())?;
        let mut transaction = client.transaction()?;
        let _ = transaction.setup();
        let _ = transaction.commit();
        benchmark! {
            group,
            "PSQL_Transaction",
            let mut transaction = client.transaction().unwrap(),
            {
                for index in 1..=1_000 {
                    transaction.each_by([index, any, any], handler);
                }
                for index in 1_001..=2_000 {
                    transaction.each_by([index, any, any], handler);
                }
                for index in 2_001..=BACKGROUND_LINKS {
                    transaction.each_by([index, any, any], handler);
                }
                let _ = transaction.commit();
            },
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
                for index in 1..=1_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 1_001..=2_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 2_001..=BACKGROUND_LINKS {
                    links.each_by([index, any, any], handler);
                }
            },
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
                for index in 1..=1_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 1_001..=2_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 2_001..=BACKGROUND_LINKS {
                    links.each_by([index, any, any], handler);
                }
            },
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
                for index in 1..=1_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 1_001..=2_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 2_001..=BACKGROUND_LINKS {
                    links.each_by([index, any, any], handler);
                }
            },
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
                for index in 1..=1_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 1_001..=2_000 {
                    links.each_by([index, any, any], handler);
                }
                for index in 2_001..=BACKGROUND_LINKS {
                    links.each_by([index, any, any], handler);
                }
            },
        }
        let _ = links.teardown();
    }
    group.finish();
    Ok(())
}
