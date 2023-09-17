#![feature(allocator_api)]
mod benchmarks;

use {
    benchmarks::create_links,
    criterion::{criterion_group, criterion_main},
};

macro_rules! tri {
    ($($body:tt)*) => {
        let _ = (|| -> linkspsql::Result<()> {
            Ok({ $($body)* })
        })().unwrap();
    };
}

pub(crate) use tri;

criterion_group!(
    benches,
    create_links,
    //delete_links,
    //each_identity,
    //each_concrete,
    //each_outgoing,
    //each_incoming,
    //each_all,
    //update_links
);
criterion_main!(benches);
