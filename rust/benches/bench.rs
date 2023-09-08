#![feature(allocator_api)]
mod benchmarks;

use {
    benchmarks::create_links,
    criterion::{criterion_group, criterion_main},
};

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
