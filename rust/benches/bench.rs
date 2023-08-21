#![feature(allocator_api)]
#![feature(box_syntax)]

use criterion::criterion_group;
use {
    create::create_links,
    criterion::criterion_main,
    delete::delete_links,
    each::{each_all, each_concrete, each_identity, each_incoming, each_outgoing},
    update::update_links,
};
mod create;
mod delete;
mod each;
mod update;

criterion_group!(
    benches,
    create_links,
    delete_links,
    each_identity,
    each_concrete,
    each_outgoing,
    each_incoming,
    each_all,
    update_links
);
criterion_main!(benches);
