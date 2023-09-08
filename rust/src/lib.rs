#![feature(allocator_api)]
#![feature(generic_associated_types)]

#[macro_export]
macro_rules! elapsed {
    {$operations:stmt} => {
        {
            let duration = Instant::now();
            $operations
            duration.elapsed()
        }
    };
}

#[macro_export]
macro_rules! benchmark {
    {$bencher:ident, $fork:stmt; $body:stmt} => {
        $bencher.iter_custom(|iters| {
            let mut time = Duration::ZERO;
            for _iter in 0..iters {
                $fork;
                time += elapsed! {
                    $body
                };
            }
            time
        });
    }
}

pub use {benched::Benched, client::Client, fork::Fork, transaction::Transaction};
use {
    doublets::{data::LinkType, mem::FileMapped},
    std::{fs::File, io, error::Error},
    tokio_postgres::NoTls,
};

mod benched;
mod client;
mod fork;
mod transaction;

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

pub const BACKGROUND_LINKS: usize = 3_000;
const OPTIONS: &str = "user=postgres dbname=postgres password=postgres host=localhost port=5432";

pub async fn connect<T: LinkType>() -> Result<Client<T>> {
    let (client, connection) = tokio_postgres::connect(OPTIONS, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {e}");
            return Err(e);
        }
        Ok(())
    });
    Client::new(client).await
}

pub fn prepare_file<T: Default>(filename: &str) -> io::Result<FileMapped<T>> {
    let file = File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(filename)?;
    FileMapped::new(file)
}

pub trait Sql {
    fn create_table(&mut self) -> Result<()>;
    fn drop_table(&mut self) -> Result<()>;
}
