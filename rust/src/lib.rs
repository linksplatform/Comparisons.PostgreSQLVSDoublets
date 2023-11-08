#![feature(allocator_api, generic_associated_types)]

#[macro_export]
macro_rules! bench {
    {|$fork:ident| as $B:ident { $($body:tt)* }} => {
        (move |bencher: &mut criterion::Bencher, benched: &mut _| {
            bencher.iter_custom(|iters| {
                let mut __bench_duration = Duration::ZERO;
                macro_rules! elapsed {
                    {$expr:expr} => {{
                        let __instant = Instant::now();
                        let __ret = {$expr};
                        __bench_duration += __instant.elapsed();
                        __ret
                    }};
                }
                crate::tri! {
                    use linkspsql::BACKGROUND_LINKS;
                    for _iter in 0..iters {
                        let mut $fork: Fork<$B> = Benched::fork(&mut *benched);
                        for _ in 0..BACKGROUND_LINKS {
                            let _ = $fork.create_point()?;
                        }
                        $($body)*
                    }
                }
                __bench_duration
            });
        })
    }
}

use doublets::Doublets;
pub use {
    benched::Benched, client::Client, exclusive::Exclusive, fork::Fork, transaction::Transaction,
};

use {
    doublets::{data::LinkType, mem::FileMapped},
    postgres::NoTls,
    std::{error, fs::File, io, result},
};

mod benched;
mod client;
mod exclusive;
mod fork;
mod transaction;

pub type Result<T, E = Box<dyn error::Error + Sync + Send>> = result::Result<T, E>;

pub const BACKGROUND_LINKS: usize = 3_000;
const PARAMS: &str = "user=postgres dbname=postgres password=postgres host=localhost port=5432";

pub fn connect<T: LinkType>() -> Result<Client<T>> {
    Client::new(postgres::Client::connect(PARAMS, NoTls)?).map_err(Into::into)
}

pub fn map_file<T: Default>(filename: &str) -> io::Result<FileMapped<T>> {
    let file = File::options().create(true).write(true).read(true).open(filename)?;
    FileMapped::new(file)
}

pub trait Sql {
    fn create_table(&mut self) -> Result<()>;
    fn drop_table(&mut self) -> Result<()>;
}
