#![feature(allocator_api)]

#[macro_export]
macro_rules! elapsed {
    {$operations:block} => {
        {
            let duration = Instant::now();
            $operations
            duration.elapsed()
        }
    };
}

#[macro_export]
macro_rules! benchmark {
    {$group:ident, $id:literal, $operations:block, $($clean:block)?} => {
        $group.bench_function($id, |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::ZERO;
                for _iter in 0..iters {
                    time += elapsed! {
                        $operations
                    };
                    $($clean)?
                }
                time
            });
        });
    };
    {$group:ident, $id:literal, $transaction:stmt, $operations:block, $($clean:block)?} => {
        $group.bench_function($id, |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::ZERO;
                for _iter in 0..iters {
                    $transaction;
                    time += elapsed! {
                        $operations
                    };
                    $($clean)?
                }
                time
            });
        });
    };
    {$group:ident, $id:literal, $operations:block, $(above $clean:block)?} => {
        $group.bench_function($id, |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::ZERO;
                for _iter in 0..iters {
                    $($clean)?
                    time += elapsed! {
                        $operations
                    };
                }
                time
            });
        });
    };
    {$group:ident, $id:literal, $transaction:stmt, $operations:block, $(above $clean:block)?} => {
        $group.bench_function($id, |b| {
            b.iter_custom(|iters| {
                let mut time = Duration::ZERO;
                for _iter in 0..iters {
                    $($clean)?
                    $transaction;
                    time += elapsed! {
                        $operations
                    };
                }
                time
            });
        });
    };
}
pub use client::Client;
use {
    doublets::{
        data::LinkType,
        mem::{Alloc, FileMapped},
        split::{self, DataPart, IndexPart},
        unit::{self, LinkPart},
        Doublets,
    },
    std::{
        alloc::Global,
        error::Error,
        fs::{remove_file, File},
        io,
    },
    tokio_postgres::NoTls,
    transaction::Transaction,
};

mod client;
mod transaction;

pub const BACKGROUND_LINKS: usize = 3_000;
const OPTIONS: &str = "user=postgres dbname=postgres password=postgres host=localhost port=5432";
pub async fn connect() -> Result<Client<usize>, Box<dyn Error>> {
    let (client, connection) = tokio_postgres::connect(OPTIONS, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {e}");
            return Err(e);
        }
        Ok(())
    });
    Client::<usize>::new(client).await
}

pub fn prepare_file(filename: &str) -> io::Result<File> {
    File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(filename)
}
pub trait SetupTeardown {
    fn setup(&mut self) -> Result<(), Box<dyn Error>>;

    fn teardown(self) -> Result<(), Box<dyn Error>>;
}

impl<T: LinkType> SetupTeardown for unit::Store<T, FileMapped<LinkPart<T>>> {
    fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..BACKGROUND_LINKS {
            self.create_point()?;
        }
        Ok(())
    }

    fn teardown(mut self) -> Result<(), Box<dyn Error>> {
        self.delete_all()?;
        remove_file("united.links")?;
        Ok(())
    }
}

impl<T: LinkType> SetupTeardown for unit::Store<T, Alloc<LinkPart<T>, Global>> {
    fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..BACKGROUND_LINKS {
            self.create_point()?;
        }
        Ok(())
    }

    fn teardown(mut self) -> Result<(), Box<dyn Error>> {
        self.delete_all()?;
        Ok(())
    }
}

impl<T: LinkType> SetupTeardown
    for split::Store<T, FileMapped<DataPart<T>>, FileMapped<IndexPart<T>>>
{
    fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..BACKGROUND_LINKS {
            self.create_point()?;
        }
        Ok(())
    }

    fn teardown(mut self) -> Result<(), Box<dyn Error>> {
        self.delete_all()?;
        remove_file("split_index.links")?;
        remove_file("split_data.links")?;
        Ok(())
    }
}

impl<T: LinkType> SetupTeardown
    for split::Store<T, Alloc<DataPart<T>, Global>, Alloc<IndexPart<T>, Global>>
{
    fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..BACKGROUND_LINKS {
            self.create_point()?;
        }
        Ok(())
    }

    fn teardown(mut self) -> Result<(), Box<dyn Error>> {
        self.delete_all()?;
        Ok(())
    }
}

impl<T: LinkType> SetupTeardown for Client<T> {
    fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..BACKGROUND_LINKS {
            self.create_point()?;
        }
        Ok(())
    }

    fn teardown(mut self) -> Result<(), Box<dyn Error>> {
        self.delete_all()?;
        self.drop_table()
    }
}

impl<T: LinkType> SetupTeardown for Transaction<'_, T> {
    fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..BACKGROUND_LINKS {
            self.create_point()?;
        }
        Ok(())
    }

    fn teardown(mut self) -> Result<(), Box<dyn Error>> {
        self.delete_all()?;
        self.drop_table()
    }
}
