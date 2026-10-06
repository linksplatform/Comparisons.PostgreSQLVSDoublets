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
                tri! {
                    let background_links = linkspsql::background_links();
                    for _iter in 0..iters {
                        let mut $fork: Fork<$B> = Benched::fork(&mut *benched);
                        for _ in 0..background_links {
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

pub use {
    benched::Benched, client::Client, exclusive::Exclusive, fork::Fork, transaction::Transaction,
};

use {
    doublets::{data::LinkReference, mem::FileMapped},
    postgres::NoTls,
    std::{env, error, fs::File, io, result},
};

mod benched;
mod client;
mod exclusive;
mod fork;
mod transaction;

pub type Result<T, E = Box<dyn error::Error + Sync + Send>> = result::Result<T, E>;

/// Number of background links to create before each benchmark iteration.
/// Configurable via BENCHMARK_BACKGROUND_LINKS environment variable.
/// Default: 1000 (for local testing), CI uses 100 for PRs and 1000 for main branch.
pub fn background_links() -> usize {
    size("BENCHMARK_BACKGROUND_LINKS", 1_000)
}

/// Number of active links; must fit in the background for update benchmarks.
pub fn benchmark_links() -> usize {
    let links = size("BENCHMARK_LINKS", 100);
    assert!(
        links <= background_links(),
        "BENCHMARK_LINKS must not exceed BENCHMARK_BACKGROUND_LINKS"
    );
    links
}

fn size(key: &str, default: usize) -> usize {
    match env::var(key) {
        Ok(value) => value
            .parse()
            .ok()
            .filter(|value| *value > 0)
            .unwrap_or_else(|| panic!("{key} must be a positive integer")),
        Err(env::VarError::NotPresent) => default,
        Err(_) => panic!("{key} must be a positive integer"),
    }
}

/// IDs created after the background links; used by both delete benchmarks.
pub fn created_links(background: usize, links: usize) -> std::ops::RangeInclusive<usize> {
    background + 1..=background + links
}

const PARAMS: &str = "user=postgres dbname=postgres password=postgres host=localhost port=5432";

pub fn connect<T: LinkReference>() -> Result<Client<T>> {
    Client::new(postgres::Client::connect(
        &env::var("POSTGRES_CONNECTION").unwrap_or_else(|_| PARAMS.to_owned()),
        NoTls,
    )?)
}

/// Converts a link value into the `i64` used by PostgreSQL `bigint` columns.
///
/// The previous `doublets` API exposed a `LinkType::as_i64` helper. The current
/// `LinkReference` trait instead provides the standard `TryInto<i64>` conversion,
/// which this function wraps for convenience.
pub fn as_i64<T: LinkReference>(value: T) -> i64 {
    value.try_into().expect("link value does not fit into i64")
}

pub fn map_file<T: Default>(filename: &str) -> io::Result<FileMapped<T>> {
    let file =
        File::options().create(true).truncate(false).write(true).read(true).open(filename)?;
    FileMapped::new(file)
}

pub trait Sql {
    fn create_table(&mut self) -> Result<()>;
    fn drop_table(&mut self) -> Result<()>;
}
