#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

pub use client::Client;
pub use cruds::Cruds;
pub use transaction::Transaction;

mod client;
mod cruds;
mod make_bench;
mod transaction;
