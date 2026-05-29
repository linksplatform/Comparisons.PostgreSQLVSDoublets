use {
    benchmarks::{
        doublets_create_links, doublets_delete_links, doublets_each_all, doublets_each_concrete,
        doublets_each_identity, doublets_each_incoming, doublets_each_outgoing,
        doublets_update_links, psql_create_links, psql_delete_links, psql_each_all,
        psql_each_concrete, psql_each_identity, psql_each_incoming, psql_each_outgoing,
        psql_update_links,
    },
    criterion::{criterion_group, criterion_main},
};

mod benchmarks;

macro_rules! tri {
    ($($body:tt)*) => {
        let _ = (|| -> linkspsql::Result<()> {
            Ok({ $($body)* })
        })().unwrap();
    };
}

pub(crate) use tri;

// PostgreSQL benchmarks
criterion_group!(
    psql_benches,
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
    doublets_benches,
    doublets_create_links,
    doublets_delete_links,
    doublets_each_identity,
    doublets_each_concrete,
    doublets_each_outgoing,
    doublets_each_incoming,
    doublets_each_all,
    doublets_update_links
);

criterion_main!(psql_benches, doublets_benches);
