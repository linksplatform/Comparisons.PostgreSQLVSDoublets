use {
    benchmarks::{
        doublets_create_links, doublets_delete_links, doublets_each_all, doublets_each_concrete,
        doublets_each_identity, doublets_each_incoming, doublets_each_outgoing,
        doublets_update_links, psql_create_links, psql_delete_links, psql_each_all,
        psql_each_concrete, psql_each_identity, psql_each_incoming, psql_each_outgoing,
        psql_update_links,
    },
    criterion::criterion_group,
};

mod benchmarks;

macro_rules! tri {
    ($($body:tt)*) => {
        let mut run = || -> linkspsql::Result<()> {
            { $($body)* };
            Ok(())
        };
        run().unwrap();
    };
}

pub(crate) use tri;

// PostgreSQL benchmarks
criterion_group!(
    name = psql_benches;
    config = configuration();
    targets =
    psql_create_links,
    psql_delete_links,
    psql_each_identity,
    psql_each_concrete,
    psql_each_outgoing,
    psql_each_incoming,
    psql_each_all,
    psql_update_links
);

// Doublets benchmarks
criterion_group!(
    name = doublets_benches;
    config = configuration();
    targets =
    doublets_create_links,
    doublets_delete_links,
    doublets_each_identity,
    doublets_each_concrete,
    doublets_each_outgoing,
    doublets_each_incoming,
    doublets_each_all,
    doublets_update_links
);

fn configuration() -> criterion::Criterion {
    criterion::Criterion::default()
        .sample_size(10)
        .warm_up_time(std::time::Duration::from_secs(1))
        .measurement_time(std::time::Duration::from_secs(1))
}

fn main() {
    linkspsql::benchmark_links();
    match std::env::var("BENCHMARK_BACKEND").as_deref().unwrap_or("all") {
        "psql" => psql_benches(),
        "doublets" => doublets_benches(),
        "all" => {
            psql_benches();
            doublets_benches();
        }
        backend => panic!("unknown BENCHMARK_BACKEND: {backend}"),
    }
    configuration().configure_from_args().final_summary();
}
